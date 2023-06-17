## GPTMiddleman

GPTMiddleman is an open-source project that provides a middleman service for the OPENAI GPT models. Please note that this project is not officially affiliated with OPENAI.

### Features
- Supports multiple GPT models, including gpt-3.5-turbo, gpt-4, and more.
- The initial caching process takes approximately one minute, while the subsequent querying process using the cache only requires a few seconds (for a dataset of 220,000 English Wikipedia articles).
- The initialization caching process takes several seconds, but querying the dataset using the cache only takes a few seconds.
- Subsequent queries for vector embeddings only take less than two seconds.

### Getting Started
To use the GPTMiddleman service, you can make API requests to the provided endpoints:

- `/v1/chat/completions` - Send a POST request to generate chat completions using the GPT models.
- `/v1/chat/json` - Send a POST request to generate chat completions using the GPT models with JSON input.
- `/v1/stream_chat/completions` - Send a POST request to generate chat completions using the GPT models via a streaming API.
- `/v1/stream_chat/json` - Send a POST request to generate chat completions using the GPT models with JSON input via a streaming API.
- `/v1/embeddings` - Send a POST request to retrieve vector embeddings for a given input text using the text-embedding-ada-002 model.

### Administration
The GPTMiddleman also provides administrative endpoints for managing user accounts:

- `/admin/create_user` - Send a POST request to create a new user account with a specified number of request counts.
- `/admin/read_users` - Send a GET request to retrieve information about all existing user accounts.
- `/admin/update_user` - Send a POST request to update the request count for a specific user account.
- `/admin/delete_users` - Send a POST request to delete one or more user accounts.

### User Management
Individual users can perform the following operations:

- `/user/read_user` - Send a POST request to retrieve information about a specific user account.
- `/user/decrement_user` - Send a POST request to decrement the request count for a specific user account.

### Usage Example
You can use cURL or any other HTTP client to send requests to the GPTMiddleman's API endpoints. Here are some examples:

Send a POST request to `/v1/chat/completions`:
```bash
curl -X POST http://localhost:3000/v1/chat/completions \
  -d '{
    "model": "gpt-3.5-turbo", 
    "messages": [{"role": "user", "content": "Say This is a test!"}]
}'
```

Send a POST request to `/v1/chat/json`:
```bash
curl -X POST http://localhost:3000/v1/chat/json \
  -d '{
    "model": "gpt-3.5-turbo", 
    "messages": [{"role": "user", "content": "Tell me a joke!"}]
}'
```

Send a POST request to `/v1/embeddings`:
```bash
curl -X POST http://localhost:3000/v1/embeddings \
  -d '{
    "input": "The food was delicious and the waiter...",
    "model": "text-embedding-ada-002"
}'
```

### Administration Example
You can also manage user accounts using the provided administrative endpoints. Here are some examples:

Send a POST request to `/admin/create_user` to create a new user:
```bash
curl -X POST http://localhost:3000/admin/create_user \
  -H "Content-Type: application/json" \
  -d '{
  "request_count": 10
}'
```

Send a GET request to `/admin/read_users` to retrieve information about all user accounts:
```bash
curl http://localhost:3000/admin/read_users
```

Send a POST request to `/admin/update_user` to update the request count for a specific user:
```bash
curl http://localhost:3000/admin/update_user \
  -d '{
  "api_token":"sk-pkNQTCFLCMCeDr0KUIPkWQMAhFiMZwXwkf4vCUFFUD5Jb05w",
  "delta": 10
}'
```

Send a POST request to `/admin/delete_users` to delete one or more user accounts:
```bash
curl http://localhost:3000/admin/delete_users \
  -H "Content-Type: application/json" \
  -d '{
  "api_tokens": ["sk-pkNQTCFLCMCeDr0KUIPkWQMAhFiMZwXwkf4vCUFFUD5Jb05w"]
}'
```

For more details and usage guidelines, please refer to the project's documentation.
