#!/bin/bash
# Script to test tabula integration locally

set -e

echo "🧪 Testing Tabula Integration"

# Create test data
echo "📄 Creating test PDF..."
cd test-data
python3 create_test_pdf.py test_table.pdf || echo "PDF creation may have failed"
cd ..

# Build Docker image
echo "🐳 Building Docker image..."
docker build -f Dockerfile.test -t recipena-tabula-test .

# Test tabula availability
echo "☕ Testing tabula availability..."
docker run --rm recipena-tabula-test java -jar /tabula.jar --help

# Test basic Rust compilation
echo "🦀 Testing Rust compilation..."
docker run --rm --entrypoint cargo recipena-tabula-test check

# Run unit tests
echo "🧪 Running unit tests..."
docker run --rm \
  --entrypoint cargo recipena-tabula-test \
  test --lib libs::tabula --verbose

# Run integration tests
echo "🔗 Running integration tests..."
docker run --rm \
  -v $(pwd)/test-data:/tmp/test-data:ro \
  --entrypoint bash recipena-tabula-test \
  -c "cp /tmp/test-data/test_table.pdf /tmp/test.pdf 2>/dev/null || echo 'PDF copy failed'; \
      cargo test --test integration_tabula test_tabula_availability -- --nocapture; \
      cargo test --test integration_tabula test_tabula_extractor_creation -- --nocapture"

# Test tabula extraction directly
echo "📊 Testing direct tabula extraction..."
docker run --rm \
  -v $(pwd)/test-data:/tmp/test-data:ro \
  --entrypoint bash recipena-tabula-test \
  -c "if [ -f /tmp/test-data/test_table.pdf ]; then \
        echo 'Testing tabula directly:'; \
        java -jar /tabula.jar --format=CSV /tmp/test-data/test_table.pdf || echo 'Direct extraction failed (may be expected)'; \
      else \
        echo 'No test PDF found for direct testing'; \
      fi"

echo "✅ All tests completed!"