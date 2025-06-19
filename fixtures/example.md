---
title: My awesome title
description: My awesome description for my awesome blog post. This is a test.
date:
    posted: 2022-01-23T11:34:00.000Z
    modified: 2022-01-26T05:26:00.000Z
coverImage: /path/to/images/cover.png
ogImage: /path/to/images/og.png
---

# My awesome title

This is a comprehensive example document showcasing various **markdown elements** that can be used in your static site generator.

## Text Formatting

You can use _italic text_ and **bold text** for emphasis. You can also combine them for **_bold italic text_**.

For code within text, use `inline code` formatting.

### Strikethrough and Emphasis

~~This text is strikethrough~~ and you can also use <sub>subscript</sub> and <sup>superscript</sup> text.

## Lists

### Unordered Lists

-   First item
-   Second item
    -   Nested item
    -   Another nested item
-   Third item

### Ordered Lists

1. First numbered item
2. Second numbered item
    1. Nested numbered item
    2. Another nested numbered item
3. Third numbered item

### Task Lists

-   [x] Completed task
-   [ ] Pending task
-   [ ] Another pending task

## Links and Images

### Links

Here's a [link to Google](https://www.google.com) and a [relative link to another page](./another-page.md).

You can also create [reference-style links][reference-link] and [another reference][another-ref].

[reference-link]: https://example.com
[another-ref]: https://example.org "Optional title"

### Images

![Alt text for image](https://via.placeholder.com/400x200/0066cc/ffffff?text=Example+Image)

![Local image](./images/local-image.png "Local image with title")

## Code Blocks

### Inline Code

Use `console.log('Hello, World!')` for inline code examples.

### Code Blocks with Syntax Highlighting

```javascript
function greet(name) {
    console.log(`Hello, ${name}!`);
    return `Welcome, ${name}`;
}

const result = greet("World");
```

```python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

print(fibonacci(10))
```

```rust
fn main() {
    println!("Hello, Rust!");

    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
}
```

### Plain Code Blocks

```
This is a plain code block without syntax highlighting
You can use it for any text that needs to be formatted as code
```

## Tables

### Basic Table

| Name  | Age | City        |
| ----- | --- | ----------- |
| Alice | 25  | New York    |
| Bob   | 30  | Los Angeles |
| Carol | 28  | Chicago     |

### Aligned Table

| Left Aligned | Center Aligned | Right Aligned |
| :----------- | :------------: | ------------: |
| Left content | Center content | Right content |
| More left    |  More center   |    More right |

## Blockquotes

> This is a blockquote. You can use it to highlight important information or quotes from other sources.

> You can also have **formatted text** within blockquotes
>
> And multiple paragraphs

## Horizontal Rules

You can separate content with horizontal rules:

---

---

---

## HTML Elements

Since this is MDX, you can also use HTML elements:

<div style="background-color: #f0f0f0; padding: 1rem; border-radius: 4px;">
    <h3>Custom HTML Block</h3>
    <p>This is a custom HTML block with styling.</p>
</div>

## Advanced Code Examples

### Shell Commands

```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build
```

### JSON Example

```json
{
    "name": "my-project",
    "version": "1.0.0",
    "dependencies": {
        "react": "^18.0.0",
        "next": "^13.0.0"
    },
    "scripts": {
        "dev": "next dev",
        "build": "next build"
    }
}
```

## Conclusion

This example demonstrates most common markdown elements you might need for your static site generator. You can customize and extend this based on your specific requirements.
