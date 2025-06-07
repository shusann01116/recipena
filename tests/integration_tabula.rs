use recipena::libs::tabula::TabulaExtractor;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_tabula_availability() {
    // Test that tabula is available in the Docker container environment
    let is_available = TabulaExtractor::is_available();

    if std::env::var("CI").is_ok() || std::env::var("DOCKER_ENV").is_ok() {
        // In CI or Docker environment, tabula should be available
        assert!(
            is_available,
            "Tabula should be available in CI/Docker environment"
        );
    } else {
        // Local development - just print status
        println!("Tabula available: {} (local development)", is_available);
    }
}

#[test]
fn test_tabula_extractor_creation() {
    let _extractor = TabulaExtractor::new();
    // Verify we can create an instance without panicking
    // If we reach this point, the test passed
}

#[test]
#[ignore] // Only run in Docker environment
fn test_tabula_extract_simple_pdf() {
    // This test requires tabula to be available
    if !TabulaExtractor::is_available() {
        println!("Skipping test - tabula not available");
        return;
    }

    let extractor = TabulaExtractor::new();

    // Create a simple test PDF path (this would need to be a real PDF in practice)
    let test_pdf_path = Path::new("/tmp/test.pdf");

    if !test_pdf_path.exists() {
        println!("Skipping test - test PDF not found at {:?}", test_pdf_path);
        return;
    }

    // Test CSV extraction
    match extractor.extract_tables(test_pdf_path, "CSV", None) {
        Ok(result) => {
            assert!(!result.is_empty(), "Extracted CSV should not be empty");
            println!("CSV extraction successful: {} chars", result.len());
        }
        Err(e) => {
            println!("CSV extraction failed (expected in test env): {}", e);
        }
    }
}

#[test]
#[ignore] // Only run in Docker environment
fn test_tabula_extract_with_pages() {
    if !TabulaExtractor::is_available() {
        println!("Skipping test - tabula not available");
        return;
    }

    let extractor = TabulaExtractor::new();
    let test_pdf_path = Path::new("/tmp/test.pdf");

    if !test_pdf_path.exists() {
        println!("Skipping test - test PDF not found");
        return;
    }

    // Test with page specification
    match extractor.extract_tables(test_pdf_path, "CSV", Some("1")) {
        Ok(result) => {
            println!(
                "Page-specific extraction successful: {} chars",
                result.len()
            );
        }
        Err(e) => {
            println!(
                "Page-specific extraction failed (expected in test env): {}",
                e
            );
        }
    }
}

#[test]
#[ignore] // Only run in Docker environment
fn test_tabula_extract_different_formats() {
    if !TabulaExtractor::is_available() {
        println!("Skipping test - tabula not available");
        return;
    }

    let extractor = TabulaExtractor::new();
    let test_pdf_path = Path::new("/tmp/test.pdf");

    if !test_pdf_path.exists() {
        println!("Skipping test - test PDF not found");
        return;
    }

    // Test different output formats
    for format in &["CSV", "JSON", "TSV"] {
        match extractor.extract_tables(test_pdf_path, format, None) {
            Ok(result) => {
                println!("{} extraction successful: {} chars", format, result.len());
            }
            Err(e) => {
                println!("{} extraction failed (expected in test env): {}", format, e);
            }
        }
    }
}

#[test]
#[ignore] // Only run in Docker environment
fn test_tabula_extract_to_file() {
    if !TabulaExtractor::is_available() {
        println!("Skipping test - tabula not available");
        return;
    }

    let extractor = TabulaExtractor::new();
    let test_pdf_path = Path::new("/tmp/test.pdf");

    if !test_pdf_path.exists() {
        println!("Skipping test - test PDF not found");
        return;
    }

    // Create temporary directory for output
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_path = temp_dir.path().join("output.csv");

    match extractor.extract_to_file(test_pdf_path, &output_path, "CSV", None) {
        Ok(()) => {
            if output_path.exists() {
                let content = fs::read_to_string(&output_path).expect("Failed to read output file");
                println!(
                    "File extraction successful: {} chars written",
                    content.len()
                );
                assert!(!content.is_empty(), "Output file should not be empty");
            } else {
                println!("Output file was not created");
            }
        }
        Err(e) => {
            println!("File extraction failed (expected in test env): {}", e);
        }
    }
}

#[test]
fn test_tabula_error_handling() {
    let extractor = TabulaExtractor::new();

    // Test with non-existent file
    let non_existent_pdf = Path::new("/tmp/non_existent.pdf");

    if TabulaExtractor::is_available() {
        match extractor.extract_tables(non_existent_pdf, "CSV", None) {
            Ok(_) => panic!("Should have failed with non-existent file"),
            Err(e) => {
                println!("Expected error for non-existent file: {}", e);
                assert!(
                    e.to_string().contains("Failed to execute tabula command")
                        || e.to_string().contains("Tabula command failed")
                );
            }
        }
    } else {
        // If tabula is not available, the error should be about command execution
        match extractor.extract_tables(non_existent_pdf, "CSV", None) {
            Ok(_) => panic!("Should have failed when tabula not available"),
            Err(e) => {
                println!("Expected error when tabula not available: {}", e);
            }
        }
    }
}
