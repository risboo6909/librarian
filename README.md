# librarian
Programming languages libraries indexer written in Rust

Uses [MeiliSearch](https://github.com/meilisearch/MeiliSearch) as a backend for storing and searching data.

All libraries are stored in JSON format and have following structure:
  
```json
{
  "id": 1,
  "name": "librarian",
  "description": "Librarian allows user to search for libraries by language, description and purpose. Fast",
  "link": "https://github.com/risboo6909/librarian",
  "target_language": "All",
  "last_commit": 1594512000,  
  "last_release": 1594511999,
  "license": "MIT",
  "usage": "web search"
}
```
