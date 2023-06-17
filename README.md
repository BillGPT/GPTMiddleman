# GPTMiddleman

The GPTMiddleman is a middleware application that serves as a middle layer between your client applications and the OpenAI API. It handles the communication with the OpenAI API, provides additional functionality such as rate limiting and user management, and simplifies the process of integrating the capabilities into your applications.

## Features

- Request forwarding: The middleman application forwards your API requests to the OpenAI GPT API and returns the responses to your client applications.
- Rate limiting: The middleman handles rate limiting and manages user quotas to prevent API abuse and control usage.
- User management: The middleman supports user management, including creating, updating, and deleting user accounts, as well as retrieving user information.
- Embeddings support: The middleman can also process and generate text embeddings using the OpenAI text embedding API.

## Usage

To use the GPTMiddleman in your project:

1. Install the dependencies: `cargo build`
2. Start the server: `cargo run`
3. Send API requests to the middleman server, specifying the desired GPT models and passing the necessary parameters.
4. Receive the responses from the middleman server and process them accordingly in your client applications.

## API Endpoints

The GPTMiddleman provides the following API endpoints:

- `POST /v1/chat/completions`: Process a chat completion request with the GPT model.
- `POST /v1/stream_chat/completions`: Process a chat completion request using streaming (continuous) responses.
- `POST /v1/chat/json`: Process a chat completion request with the GPT model, using JSON format.
- `POST /v1/stream_chat/json`: Process a chat completion request using streaming (continuous) responses, using JSON format.
- `POST /v1/embeddings`: Process a text embedding request using the GPT `text-embedding-ada-002` model.
- `POST /admin/create_user`: Create a new user with a specified request count.
- `GET /admin/read_users`: Read the list of all users and their request counts.
- `POST /admin/update_user`: Update the request count of a specific user.
- `POST /admin/delete_users`: Delete multiple users by their API tokens.
- `POST /user/read_user`: Read the request count of a specific user.
- `POST /user/decrement_user`: Decrement the request count of a specific user.

## Example API Usage

Here are some examples of how to use the GPTMiddleman API:

1. Process a chat completion request:
   ```bash
   curl -X POST http://localhost:3000/v1/chat/completions \
     -d '{
       "model": "gpt-3.5-turbo", 
       "messages": [{"role": "user", "content": "Say This is a test!"}]
     }'
   ```
2. Process a streaming chat completion request:
   ```bash
   curl -X POST http://localhost:3000/v1/stream_chat/completions \
     -d '{
       "model": "gpt-3.5-turbo", 
       "messages": [{"role": "user", "content": "Tell me a story!"}]
     }'
   ```
3. Process a chat completion request with JSON format:
   ```bash
   curl -X POST http://localhost:3000/v1/chat/json \
     -d '{
       "model": "gpt-3.5-turbo",
       "messages": [{"role": "user", "content": "Say This is a test!"}]
     }'
   ```
4. Process a streaming chat completion request with JSON format:
   ```bash
   curl -X POST http://localhost:3000/v1/stream_chat/json \
     -d '{
       "model": "gpt-3.5-turbo",
       "messages": [{"role": "user", "content": "Tell me a story!"}]
     }'
   ```
5. Process a text embedding request:
   ```bash
   curl -X POST http://localhost:3000/v1/embeddings \
     -d '{
       "input": "The food was delicious and the waiter was very attentive.",
       "model": "text-embedding-ada-002"
     }'
   ```
6. Create a new user:
   ```bash
   curl -X POST http://localhost:3000/admin/create_user \
     -H "Content-Type: application/json" \
     -d '{
       "request_count": 10
     }'
   ```
7. Read the list of all users:
   ```bash
   curl http://localhost:3000/admin/read_users
   ```
8. Update the request count of a specific user:
   ```bash
   curl http://localhost:3000/admin/update_user \
     -d '{
       "api_token": "sk-S8XbAUS6B14pvpiomaajtwWSuwS58EYw7POP9VNSffC4Ykx6",
       "delta": 10
     }'
   ```
9. Delete multiple users by their API tokens:
   ```bash
   curl http://localhost:3000/admin/delete_users \
     -H "Content-Type: application/json" \
     -d '{
       "api_tokens": ["sk-yGDpHLEzHErHPXjhpUk6CIBzdVYxUInUUQArHNt4aC925vCe", "sk-hlK5uPXWmKCJ4PjlY6QiQtnoKepvlJWsSdJJkL9VJWRLwkM7"]
     }'
   ```
10. Read the request count of a specific user:
   ```bash
   curl http://localhost:3000/user/read_user \
     -d '{
       "api_token": "sk-S8XbAUS6B14pvpiomaajtwWSuwS58EYw7POP9VNSffC4Ykx6"
     }'
   ```
11. Decrement the request count of a specific user:
   ```bash
   curl http://localhost:3000/user/decrement_user \
     -d '{
       "api_token": "sk-S8XbAUS6B14pvpiomaajtwWSuwS58EYw7POP9VNSffC4Ykx6"
     }'
   ```

Please note that in the above commands, replace `http://localhost:3000` with the actual address and port of your server. Also, note that the POST requests are using the `-d` option with an empty data body (`""`) as the sample routes do not use request bodies.
