#!/usr/bin/env python3
"""
Simple PDF Text Extractor for Squeak Language Reference
"""

import os
import sys
from pathlib import Path
import re

def extract_pdf_content(pdf_path):
    """Extract text from PDF using available libraries"""
    try:
        import pypdf
        
        with open(pdf_path, 'rb') as file:
            reader = pypdf.PdfReader(file)
            text = ""
            
            print(f"📖 PDF has {len(reader.pages)} pages")
            
            for i, page in enumerate(reader.pages):
                try:
                    page_text = page.extract_text()
                    if page_text:
                        text += f"\n=== PAGE {i+1} ===\n"
                        text += page_text
                        text += "\n"
                except Exception as e:
                    print(f"⚠️  Error extracting page {i+1}: {e}")
            
            return text
            
    except Exception as e:
        print(f"❌ PDF extraction failed: {e}")
        return None

def clean_filename(name):
    """Clean filename for Windows filesystem"""
    # Remove invalid characters
    name = re.sub(r'[<>:"/\\|?*]', '_', name)
    # Remove extra spaces and limit length
    name = re.sub(r'\s+', '_', name)[:50]
    return name.strip('_')

def save_content(content):
    """Save the extracted content"""
    ref_dir = Path("doc/language_reference")
    ref_dir.mkdir(exist_ok=True)
    
    # Save full content
    with open(ref_dir / "squeak_language_reference_full.txt", 'w', encoding='utf-8') as f:
        f.write(content)
    
    # Try to identify and save sections
    lines = content.split('\n')
    sections = {}
    current_section = "introduction"
    current_content = []
    
    for line in lines:
        line_clean = line.strip()
        
        # Simple section detection
        if (line_clean and 
            (line_clean.isupper() or 
             any(word in line_clean.lower() for word in [
                 'chapter', 'section', 'syntax', 'grammar', 'language',
                 'expressions', 'statements', 'classes', 'methods', 'objects',
                 'literals', 'variables', 'messages', 'blocks'
             ])) and 
            len(line_clean) < 80):
            
            # Save previous section
            if current_content and len('\n'.join(current_content).strip()) > 100:
                sections[current_section] = '\n'.join(current_content)
            
            # Start new section
            current_section = clean_filename(line_clean.lower())
            current_content = [line]
        else:
            current_content.append(line)
    
    # Save final section
    if current_content and len('\n'.join(current_content).strip()) > 100:
        sections[current_section] = '\n'.join(current_content)
    
    # Save sections
    for section_name, section_content in sections.items():
        filename = f"{section_name}.md"
        try:
            with open(ref_dir / filename, 'w', encoding='utf-8') as f:
                f.write(f"# {section_name.replace('_', ' ').title()}\n\n")
                f.write(section_content)
        except Exception as e:
            print(f"⚠️  Could not save section {section_name}: {e}")
    
    return sections

def main():
    print("🔍 Squeak Language Reference Extractor (Simple)")
    print("=" * 55)
    
    pdf_path = Path("doc/SqueakLanguageRef.1.pdf")
    if not pdf_path.exists():
        print(f"❌ PDF not found: {pdf_path}")
        return
    
    print(f"📄 Extracting from: {pdf_path}")
    
    content = extract_pdf_content(pdf_path)
    
    if not content:
        print("❌ Could not extract text from PDF")
        return
    
    print(f"✅ Extracted {len(content)} characters")
    
    sections = save_content(content)
    
    print(f"\n📁 Created files:")
    print(f"   • Full content: doc/language_reference/squeak_language_reference_full.txt")
    print(f"   • Sections: {len(sections)} files")
    
    # Show preview
    lines = [line for line in content.split('\n')[:30] if line.strip()]
    print(f"\n📖 Preview:")
    print("-" * 50)
    for i, line in enumerate(lines[:15], 1):
        print(f"{i:2d}: {line}")

if __name__ == "__main__":
    main()