use anyhow::{Context, Result};
use std::path::Path;
use std::process::{Command, Stdio};

pub struct TabulaExtractor;

impl Default for TabulaExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl TabulaExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Extract tables from PDF using tabula
    ///
    /// # Arguments
    /// * `pdf_path` - Path to the PDF file
    /// * `output_format` - Output format ("CSV", "JSON", "TSV")
    /// * `pages` - Optional page numbers to extract (e.g., "1-3" or "1,3,5")
    ///
    /// # Returns
    /// Returns the extracted table data as a String
    pub fn extract_tables(
        &self,
        pdf_path: &Path,
        output_format: &str,
        pages: Option<&str>,
    ) -> Result<String> {
        let mut cmd = Command::new("java");
        cmd.args(["-jar", "/tabula.jar"]);

        // Set output format
        match output_format.to_uppercase().as_str() {
            "CSV" => cmd.arg("--format=CSV"),
            "JSON" => cmd.arg("--format=JSON"),
            "TSV" => cmd.arg("--format=TSV"),
            _ => cmd.arg("--format=CSV"), // Default to CSV
        };

        // Set pages if specified
        if let Some(page_range) = pages {
            cmd.arg("--pages").arg(page_range);
        }

        // Add PDF file path
        cmd.arg(pdf_path);

        // Configure command
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        // Execute command
        let output = cmd.output().context("Failed to execute tabula command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Tabula command failed: {}", stderr));
        }

        let result =
            String::from_utf8(output.stdout).context("Failed to convert tabula output to UTF-8")?;

        Ok(result)
    }

    /// Extract tables and save to file
    pub fn extract_to_file(
        &self,
        pdf_path: &Path,
        output_path: &Path,
        output_format: &str,
        pages: Option<&str>,
    ) -> Result<()> {
        let mut cmd = Command::new("java");
        cmd.args(["-jar", "/tabula.jar"]);

        // Set output format
        match output_format.to_uppercase().as_str() {
            "CSV" => cmd.arg("--format=CSV"),
            "JSON" => cmd.arg("--format=JSON"),
            "TSV" => cmd.arg("--format=TSV"),
            _ => cmd.arg("--format=CSV"),
        };

        // Set pages if specified
        if let Some(page_range) = pages {
            cmd.arg("--pages").arg(page_range);
        }

        // Set output file
        cmd.arg("--outfile").arg(output_path);

        // Add PDF file path
        cmd.arg(pdf_path);

        // Execute command
        let output = cmd.output().context("Failed to execute tabula command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Tabula command failed: {}", stderr));
        }

        Ok(())
    }

    /// Check if tabula is available in the system
    pub fn is_available() -> bool {
        Command::new("java")
            .args(["-jar", "/tabula.jar", "--help"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tabula_extractor_creation() {
        let _extractor = TabulaExtractor::new();
        // Test that we can create an instance without panicking
        // If we reach this point, the test passed
    }

    #[test]
    fn test_tabula_availability() {
        // This test will pass in CI with Docker container, but may fail locally
        let is_available = TabulaExtractor::is_available();
        println!("Tabula available: {}", is_available);
        // Don't assert here as it depends on environment
    }
}
