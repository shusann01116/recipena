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
	// Fetch required environment variables and check for missing values.
	channelSecret := os.Getenv("LINE_CHANNEL_SECRET")
	if channelSecret == "" {
		log.Fatal("Environment variable LINE_CHANNEL_SECRET is not set")
	}

	channelToken := os.Getenv("LINE_CHANNEL_TOKEN")
	if channelToken == "" {
		log.Fatal("Environment variable LINE_CHANNEL_TOKEN is not set")
	}

	integrationToken := os.Getenv("NOTION_INTEGRATION_TOKEN")
	if integrationToken == "" {
		log.Fatal("Environment variable NOTION_INTEGRATION_TOKEN is not set")
	}

	databaseID := os.Getenv("NOTION_DATABASE_ID")
	if databaseID == "" {
		log.Fatal("Environment variable NOTION_DATABASE_ID is not set")
	}

	// Initialize the Line messaging bot.
	bot, err := messaging_api.NewMessagingApiAPI(channelToken)
	if err != nil {
		log.Fatal(err)
	}

	// Initialize the Notion client.
	client := notionapi.NewClient(notionapi.Token(integrationToken))
	messageHandler := makeMessageHandler(client, databaseID)

	// Register the HTTP Cloud Function.
	functions.HTTP("LineWebhook", makeLineWebhook(channelSecret, bot, messageHandler))
}

// MessageHandlerFunc defines a function to handle a LINE text message.
type MessageHandlerFunc func(ctx context.Context, message webhook.TextMessageContent) error

// sendReply sends a text reply using the provided bot and replyToken.
func sendReply(bot *messaging_api.MessagingApiAPI, replyToken, text string) error {
	_, err := bot.ReplyMessage(&messaging_api.ReplyMessageRequest{
		ReplyToken: replyToken,
		Messages: []messaging_api.MessageInterface{
			messaging_api.TextMessage{Text: text},
		},
	})
	return err
}

// makeLineWebhook creates the HTTP handler for the LINE webhook.
func makeLineWebhook(channelSecret string, bot *messaging_api.MessagingApiAPI, messageHandler MessageHandlerFunc) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		ctx := r.Context()

		cb, err := webhook.ParseRequest(channelSecret, r)
		if err != nil {
			log.Printf("Cannot parse request: %+v\n", err)
			if errors.Is(err, webhook.ErrInvalidSignature) {
				w.WriteHeader(http.StatusBadRequest)
			} else {
				w.WriteHeader(http.StatusInternalServerError)
			}
			return
		}

		log.Println("Handling events...")
		for _, event := range cb.Events {
			log.Printf("Received event: %+v\n", event)

			switch e := event.(type) {
			case webhook.MessageEvent:
				switch message := e.Message.(type) {
				case webhook.TextMessageContent:
					// Check if the text is a valid URL.
					if _, err := url.ParseRequestURI(message.Text); err != nil {
						if errReply := sendReply(bot, e.ReplyToken, "URLを入力してね"); errReply != nil {
							log.Print(errReply)
						}
						// Stop processing if URL is invalid.
						return
					}

					// Process the message by creating a Notion page.
					if err := messageHandler(ctx, message); err != nil {
						log.Print(err)
						// Optionally reply with an error message.
						if errReply := sendReply(bot, e.ReplyToken, "Notionへの登録に失敗しました。"); errReply != nil {
							log.Print(errReply)
						}
						return
					}

					if err := sendReply(bot, e.ReplyToken, "Notionに登録したよ ✨"); err != nil {
						log.Print(err)
					} else {
						log.Println("Sent text reply.")
					}
				case webhook.StickerMessageContent:
					replyText := fmt.Sprintf("sticker id is %s, stickerResourceType is %s", message.StickerId, message.StickerResourceType)
					if err := sendReply(bot, e.ReplyToken, replyText); err != nil {
						log.Print(err)
					} else {
						log.Println("Sent sticker reply.")
					}
				default:
					log.Printf("Unsupported message content: %T\n", e.Message)
				}
			default:
				log.Printf("Unsupported event type: %T\n", event)
			}
		}
	}
}

// makeMessageHandler returns a function that handles a LINE text message
// by creating a new page in Notion with the page title and URL.
func makeMessageHandler(client *notionapi.Client, databaseID string) MessageHandlerFunc {
	return func(ctx context.Context, message webhook.TextMessageContent) error {
		// Verify the message is a valid URL.
		u, err := url.ParseRequestURI(message.Text)
		if err != nil {
			return err
		}
		// Get the title of the webpage.
		title, err := getTitleFromURL(u)
		if err != nil {
			return err
		}

		// Create a new page in Notion.
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
		return err
	}
}

// getTitleFromURL retrieves and parses the HTML page at the given URL and extracts the <title> element.
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

	doc, err := html.Parse(bytes.NewReader(body))
	if err != nil {
		return "", err
	}

	var title string
	// traverse recursively to locate the <title> element.
	var traverse func(*html.Node)
	traverse = func(n *html.Node) {
		if title != "" {
			// Early stop if title is found.
			return
		}
		if n.Type == html.ElementNode && n.Data == "title" && n.FirstChild != nil {
			title = n.FirstChild.Data
			return
		}
		for c := n.FirstChild; c != nil; c = c.NextSibling {
			traverse(c)
			if title != "" {
				return
			}
		}
	}
	traverse(doc)

	if title == "" {
		return "", errors.New("unable to locate title on the page")
	}
	return title, nil
}
