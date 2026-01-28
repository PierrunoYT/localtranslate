# LocalTranslate

A minimal, local translation application built with Tauri, React, and TypeScript. Translate text between multiple languages with a clean, intuitive interface.

## Features

- 🌍 Multi-language support (English, Spanish, French, German, Italian, Portuguese, Japanese, Chinese)
- 🔄 Quick language swap functionality
- 🎨 Clean, minimal UI with dark mode support
- 📱 Responsive design for various screen sizes
- ⚡ Fast and lightweight desktop application
- 🔒 Local processing (privacy-focused)

## Tech Stack

- **Frontend**: React 19 + TypeScript
- **Framework**: Tauri 2
- **Build Tool**: Vite 7
- **Styling**: CSS3 with CSS Variables

## Prerequisites

- Node.js (v18 or higher)
- Rust (latest stable)
- npm or yarn

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd LocalTranslator/localtranslate
```

2. Install dependencies:
```bash
npm install
```

## Development

Run the development server:
```bash
npm run dev
```

Run with Tauri (desktop app):
```bash
npm run tauri dev
```

## Building

Build the application:
```bash
npm run build
```

Build the Tauri desktop app:
```bash
npm run tauri build
```

The built application will be in `src-tauri/target/release/`.

## Project Structure

```
localtranslate/
├── src/                 # React frontend source
│   ├── App.tsx         # Main application component
│   ├── App.css         # Application styles
│   └── main.tsx        # React entry point
├── src-tauri/          # Rust backend source
│   └── src/            # Rust source files
├── public/             # Static assets
└── package.json        # Node.js dependencies
```

## Usage

1. Select the source language from the dropdown
2. Select the target language from the dropdown
3. Enter text in the left text area
4. Click "Translate" to see the translation in the right text area
5. Use the swap button (⇄) to quickly swap source and target languages

## Supported Languages

- English (en)
- Spanish (es)
- French (fr)
- German (de)
- Italian (it)
- Portuguese (pt)
- Japanese (ja)
- Chinese (zh)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
