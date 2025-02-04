package recipena

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"io"
	"log"
	"net/http"
	"net/url"
	"os"

	"github.com/GoogleCloudPlatform/functions-framework-go/functions"
	"github.com/jomei/notionapi"
	"github.com/line/line-bot-sdk-go/v8/linebot/messaging_api"
	"github.com/line/line-bot-sdk-go/v8/linebot/webhook"
	"golang.org/x/net/html"
)

func init() {
	channelSecret := os.Getenv("LINE_CHANNEL_SECRET")
	bot, err := messaging_api.NewMessagingApiAPI(
		os.Getenv("LINE_CHANNEL_TOKEN"),
	)
	integrationToken := os.Getenv("NOTION_INTEGRATION_TOKEN")
	client := notionapi.NewClient(notionapi.Token(integrationToken))
	databaseID := os.Getenv("NOTION_DATABASE_ID")
	messageHandler := makeMessageHandler(client, databaseID)

	if err != nil {
		log.Fatal(err)
	}
	functions.HTTP("LineWebhook", makeLineWebhook(channelSecret, bot, messageHandler))
}

type MessageHandlerFunc func(ctx context.Context, message webhook.TextMessageContent) error

// makeLineWebhook is an HTTP Cloud Function.
func makeLineWebhook(channelSecret string, bot *messaging_api.MessagingApiAPI, messageHandler MessageHandlerFunc) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		ctx := r.Context()
		cb, err := webhook.ParseRequest(channelSecret, r)
		if err != nil {
			log.Printf("Cannot parse request: %+v\n", err)
			if errors.Is(err, webhook.ErrInvalidSignature) {
				w.WriteHeader(400)
			} else {
				w.WriteHeader(500)
			}
			return
		}

		log.Println("Handling events...")
		for _, event := range cb.Events {
			log.Printf("webhook called%+v...\n", event)

			switch e := event.(type) {
			case webhook.MessageEvent:
				switch message := e.Message.(type) {
				case webhook.TextMessageContent:
					if _, err := url.Parse(message.Text); err != nil {
						if _, err = bot.ReplyMessage(
							&messaging_api.ReplyMessageRequest{
								ReplyToken: e.ReplyToken,
								Messages: []messaging_api.MessageInterface{
									messaging_api.TextMessage{
										Text: "URLを入力してね",
									},
								},
							},
						); err != nil {
							log.Print(err)
							return
						}
					}
					if err := messageHandler(ctx, message); err != nil {
						log.Print(err)
						return
					}
					if _, err = bot.ReplyMessage(
						&messaging_api.ReplyMessageRequest{
							ReplyToken: e.ReplyToken,
							Messages: []messaging_api.MessageInterface{
								messaging_api.TextMessage{
									Text: "Notionに登録したよ ✨",
								},
							},
						},
					); err != nil {
						log.Print(err)
						return
					} else {
						log.Println("Sent text reply.")
					}
				case webhook.StickerMessageContent:
					replyMessage := fmt.Sprintf(
						"sticker id is %s, stickerResourceType is %s", message.StickerId, message.StickerResourceType)
					if _, err = bot.ReplyMessage(
						&messaging_api.ReplyMessageRequest{
							ReplyToken: e.ReplyToken,
							Messages: []messaging_api.MessageInterface{
								messaging_api.TextMessage{
									Text: replyMessage,
								},
							},
						}); err != nil {
						log.Print(err)
					} else {
						log.Println("Sent sticker reply.")
					}
				default:
					log.Printf("Unsupported message content: %T\n", e.Message)
				}
			default:
				log.Printf("Unsupported message: %T\n", event)
			}
		}
	}
}

func makeMessageHandler(client *notionapi.Client, databaseID string) MessageHandlerFunc {
	return func(ctx context.Context, message webhook.TextMessageContent) error {
		// Verify the message is a valid URL
		u, err := url.Parse(message.Text)
		if err != nil {
			return err
		}
		// get the title from the url
		title, err := getTitleFromURL(u)
		if err != nil {
			return err
		}

		// Create a new page in Notion
		_, err = client.Page.Create(ctx, &notionapi.PageCreateRequest{
			Parent: notionapi.Parent{
				DatabaseID: notionapi.DatabaseID(databaseID),
			},
			Properties: map[string]notionapi.Property{
				"Name": notionapi.TitleProperty{
					Title: []notionapi.RichText{
						{
							Text: &notionapi.Text{Content: title},
						},
					},
				},
				"リンク": notionapi.URLProperty{
					URL: message.Text,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	}
}

func getTitleFromURL(u *url.URL) (string, error) {
	resp, err := http.Get(u.String())
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", err
	}

	// parse the body as html
	doc, err := html.Parse(bytes.NewReader(body))
	if err != nil {
		return "", err
	}

	// find the title tag
	var title string
	var f func(*html.Node)
	f = func(n *html.Node) {
		if n.Type == html.ElementNode && n.Data == "title" {
			title = n.FirstChild.Data
		}
		for c := n.FirstChild; c != nil; c = c.NextSibling {
			f(c)
		}
	}
	f(doc)
	return title, nil
}
