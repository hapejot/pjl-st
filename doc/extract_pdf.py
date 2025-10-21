#!/usr/bin/env python3
"""
PDF Text Extractor for Squeak Language Reference
Extracts content from the Squeak Language Reference PDF
"""

import os
import sys
from pathlib import Path

def check_pdf_libraries():
    """Check if required PDF libraries are available"""
    missing = []
    
    try:
        import PyPDF2
        print("✓ PyPDF2 available")
    except ImportError:
        try:
            import pypdf
            print("✓ pypdf available")
        except ImportError:
            missing.append("pypdf")
    
    try:
        import pdfplumber
        print("✓ pdfplumber available")
    except ImportError:
        missing.append("pdfplumber")
    
    if missing:
        print(f"\n❌ Missing packages: {', '.join(missing)}")
        print("\nInstall with:")
        print(f"pip install {' '.join(missing)}")
        return False
    
    return True

def extract_with_pypdf(pdf_path):
    """Extract text using PyPDF2/pypdf"""
    try:
        # Try new pypdf first
        try:
            import pypdf as pdf_lib
            reader_class = pdf_lib.PdfReader
        except ImportError:
            # Fall back to PyPDF2
            import PyPDF2 as pdf_lib
            reader_class = pdf_lib.PdfReader
        
        with open(pdf_path, 'rb') as file:
            reader = reader_class(file)
            text = ""
            
            print(f"📖 PDF has {len(reader.pages)} pages")
            
            for i, page in enumerate(reader.pages):
                try:
                    page_text = page.extract_text()
                    if page_text:
                        text += f"\n--- PAGE {i+1} ---\n"
                        text += page_text
                        text += "\n"
                except Exception as e:
                    print(f"⚠️  Error extracting page {i+1}: {e}")
            
            return text
            
    except Exception as e:
        print(f"❌ pypdf extraction failed: {e}")
        return None

def extract_with_pdfplumber(pdf_path):
    """Extract text using pdfplumber (better formatting)"""
    try:
        import pdfplumber
        
        text = ""
        with pdfplumber.open(pdf_path) as pdf:
            print(f"📖 PDF has {len(pdf.pages)} pages")
            
            for i, page in enumerate(pdf.pages):
                try:
                    page_text = page.extract_text()
                    if page_text:
                        text += f"\n--- PAGE {i+1} ---\n"
                        text += page_text
                        text += "\n"
                        
                    # Also try to extract tables if any
                    tables = page.extract_tables()
                    for j, table in enumerate(tables):
                        text += f"\n--- TABLE {j+1} ON PAGE {i+1} ---\n"
                        for row in table:
                            text += " | ".join(str(cell) if cell else "" for cell in row) + "\n"
                        text += "\n"
                        
                except Exception as e:
                    print(f"⚠️  Error extracting page {i+1}: {e}")
        
        return text
        
    except Exception as e:
        print(f"❌ pdfplumber extraction failed: {e}")
        return None

def create_language_reference_structure(content):
    """Create organized files from the extracted content"""
    
    # Create directory structure
    ref_dir = Path("doc/language_reference")
    ref_dir.mkdir(exist_ok=True)
    
    # Save full content
    with open(ref_dir / "full_content.txt", 'w', encoding='utf-8') as f:
        f.write(content)
    
    # Try to split into sections based on common patterns
    sections = {}
    current_section = "introduction"
    current_content = []
    
    lines = content.split('\n')
    
    for line in lines:
        line = line.strip()
        
        # Look for chapter/section headers
        if any(keyword in line.lower() for keyword in [
            'chapter', 'section', 'syntax', 'grammar', 'semantics',
            'expressions', 'statements', 'classes', 'methods', 'objects'
        ]) and len(line) < 100:  # Likely a header
            
            # Save previous section
            if current_content:
                sections[current_section] = '\n'.join(current_content)
            
            # Start new section
            current_section = line.lower().replace(' ', '_').replace('/', '_')
            current_content = [line]
        else:
            current_content.append(line)
    
    # Save final section
    if current_content:
        sections[current_section] = '\n'.join(current_content)
    
    # Save individual sections
    for section_name, section_content in sections.items():
        filename = f"{section_name}.md"
        with open(ref_dir / filename, 'w', encoding='utf-8') as f:
            f.write(f"# {section_name.replace('_', ' ').title()}\n\n")
            f.write(section_content)
    
    print(f"\n📁 Created {len(sections)} sections in {ref_dir}")
    return sections

def main():
    print("🔍 Squeak Language Reference Extractor")
    print("=" * 50)
    
    pdf_path = Path("doc/SqueakLanguageRef.1.pdf")
    if not pdf_path.exists():
        print(f"❌ PDF not found: {pdf_path}")
        return
    
    if not check_pdf_libraries():
        return
    
    print(f"\n📄 Extracting from: {pdf_path}")
    
    # Try pdfplumber first (better formatting)
    content = extract_with_pdfplumber(pdf_path)
    
    # Fall back to pypdf if pdfplumber fails
    if not content:
        print("🔄 Trying pypdf as fallback...")
        content = extract_with_pypdf(pdf_path)
    
    if not content:
        print("❌ Could not extract text from PDF")
        return
    
    print(f"✅ Extracted {len(content)} characters")
    
    # Create organized structure
    sections = create_language_reference_structure(content)
    
    print("\n📋 Summary:")
    print(f"   • Full content: doc/language_reference/full_content.txt")
    print(f"   • Sections: {len(sections)} files created")
    
    # Show first few lines as preview
    lines = content.split('\n')[:20]
    print(f"\n📖 Preview (first 20 lines):")
    print("-" * 50)
    for i, line in enumerate(lines, 1):
        if line.strip():
            print(f"{i:2d}: {line}")

if __name__ == "__main__":
    main()