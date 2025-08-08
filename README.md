# CSS Structs

[![Crates.io](https://img.shields.io/crates/v/css-parser.svg)](https://crates.io/crates/css-structs)
[![Documentation](https://docs.rs/css-parser/badge.svg)](https://docs.rs/css-structs)

A fast and reliable CSS parser for Rust, built with [nom](https://github.com/Geal/nom) parser combinators. Parse CSS stylesheets, rules, declarations, and declaration lists with ease.

## Features

- 🚀 **Fast**: Built with nom parser combinators for excellent performance
- 🔧 **Flexible**: Parse complete stylesheets or individual components
- 🔄 **Roundtrip**: Parse CSS and convert back to string representation

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
css-structs = "1.0.0"
```

## Usage

### Parse a Complete Stylesheet

```rust
use css_structs::Stylesheet;

let css = r#"
    body {
        margin: 0;
        padding: 0;
        font-family: Arial, sans-serif;
    }

    .header {
        background-color: #333;
        color: white;
        padding: 1rem;
    }

    .button {
        background: linear-gradient(45deg, #007bff, #0056b3);
        border: none !important;
        padding: 0.5rem 1rem;
    }
"#;

let stylesheet = Stylesheet::from_string(css).unwrap();
println!("Parsed {} CSS rules", stylesheet.rules.len());

// Convert back to CSS string
println!("{}", stylesheet);
```

### Parse Individual CSS Rules

```rust
use css_structs::CSSRule;

let rule_str = ".container { max-width: 1200px; margin: 0 auto; }";
let rule = CSSRule::from_string(rule_str).unwrap();

println!("Selector: {}", rule.selector);
println!("Declarations: {}", rule.declarations.declarations.len());
```

### Parse Declaration Lists

```rust
use css_structs::CSSDeclarationList;

let declarations = "color: red; margin: 10px; padding: 5px !important";
let list = CSSDeclarationList::from_string(declarations).unwrap();

// Remove a specific declaration
let mut list = list;
list.remove_declaration("color");
println!("{}", list); // "margin: 10px; padding: 5px !important;"
```

### Parse Individual Declarations

```rust
use css_structs::CSSDeclaration;

let decl = CSSDeclaration::from_string("font-size: 16px !important").unwrap();
println!("Property: {}", decl.property);
println!("Value: {}", decl.value);
println!("Important: {}", decl.important.is_some());
```

### Working with Parsed Data

```rust
use css_structs::{Stylesheet, CSSDeclaration};

let css = r#"
    .card { 
        background: white;
        border: 1px solid #ddd;
        border-radius: 4px;
        padding: 1rem;
    }
"#;

let stylesheet = Stylesheet::from_string(css).unwrap();
let rule = &stylesheet.rules[0];

// Access rule components
println!("Selector: {}", rule.selector);

// Iterate through declarations
for declaration in &rule.declarations.declarations {
    println!("  {}: {}", declaration.property, declaration.value);
    if declaration.important.is_some() {
        println!("    (important)");
    }
}

// Modify and rebuild
let mut new_declarations = rule.declarations.clone();
new_declarations.declarations.push(
    CSSDeclaration::new("box-shadow", "0 2px 4px rgba(0,0,0,0.1)", None)
);
```

## API Overview

### Core Types

- **`Stylesheet`** - Represents a complete CSS stylesheet with multiple rules
- **`CSSRule`** - Represents a single CSS rule (selector + declarations)
- **`CSSDeclarationList`** - Represents a list of CSS declarations
- **`CSSDeclaration`** - Represents a single CSS property-value pair

### Key Methods

- `from_string()` - Parse from CSS string (available on all types)
- `new()` - Create instances programmatically
- `remove_declaration()` - Remove declarations by property name (CSSDeclarationList)
- `Display` trait - Convert back to CSS string format

## CSS Features Supported

- ✅ Basic selectors (element, class, ID, universal)
- ✅ Complex selectors (descendant, child, sibling, pseudo-classes)
- ✅ All CSS properties and values
- ✅ `!important` declarations
- ✅ Vendor prefixes (`-webkit-`, `-moz-`, etc.)
- ✅ CSS custom properties (CSS variables)
- ✅ Whitespace handling and normalization
- ❌ Comments - *not supported yet*
- ❌ At-rules (e.g., `@media`, `@font-face`, `@keyframes`) - *not supported yet*
- ❌ Nested rules - *not supported yet*

## Error Handling

The parser returns `Result` types for graceful error handling:

```rust
use css_structs::Stylesheet;

let invalid_css = "body { color: red; margin: 10px"; // missing closing brace
match Stylesheet::from_string(invalid_css) {
    Ok(stylesheet) => println!("Parsed successfully!"),
    Err(error) => eprintln!("Parse error: {}", error),
}
```

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run specific test module
cargo test stylesheet::tests
```

