# GPTMiddleman

GPTMiddleman is a Rust-based server application that serves as a middleman between your application and OpenAI's GPT-3 and GPT-4 models. It provides a set of RESTful APIs that allow you to interact with the GPT models, including generating chat completions, streaming chat completions, and retrieving embeddings.

In addition to the GPT model interaction, GPTMiddleman also provides a set of administrative APIs for managing users, including creating, reading, updating, and deleting users.

## Features

- **GPT-3 and GPT-4 Support**: GPTMiddleman supports both GPT-3 and GPT-4 models, allowing you to generate chat completions, stream chat completions, and retrieve embeddings.

- **User Management**: GPTMiddleman provides a set of administrative APIs for managing users, including creating, reading, updating, and deleting users.

- **In-Memory Cache**: GPTMiddleman uses an in-memory cache to store embeddings data, improving the performance of the application.

- **Vector Search**: GPTMiddleman provides a vector search functionality that allows you to find the top N most similar vectors in the embeddings data.

- **SQLite Database**: GPTMiddleman uses SQLite for storing user data, providing a lightweight and efficient solution for user management.

## Getting Started(Linux)

To get started with GPTMiddleman, you need to have Rust installed on your machine. You can download Rust from the official website.

Install dependencies:

```bash
sudo apt install rustc
sudo apt install cargo
sudo apt install libssl-dev
sudo apt install libsqlite3-dev
```

Once you have Rust installed, you can clone the GPTMiddleman repository and build the project using the following commands:

```bash
git clone https://github.com/yourusername/GPTMiddleman.git
cd GPTMiddleman
cargo build --release
```

After building the project, you can start the server using the following command:
```bash
cargo run --release
```

The server will start on port 3000.

## API Documentation
To send HTTP requests through the terminal, you can use the curl command-line tool. Here are examples of using curl to send different types of HTTP requests:

Send a POST request to /v1/chat/completions:
```bash
curl -X POST http://localhost:3000/v1/chat/completions \
  -d '{
    "model": "gpt-3.5-turbo", 
    "messages": [{"role": "user", "content": "Say This is a test!"}]
}'
```
Send a POST request to /v1/chat/completions:
```bash
curl -X POST http://localhost:3000/v1/chat/json \
  -d '{
    "model": "gpt-3.5-turbo", 
    "messages": [{"role": "user", "content": "介绍一下Donald Trump"}]
}'
```
Send a POST request to /v1/stream_chat/completions:
```bash
curl -X POST http://localhost:3000/v1/stream_chat/completions \
  -d '{
    "model": "gpt-3.5-turbo", 
    "messages": [{"role": "user", "content": "讲一个故事!"}]
}'
```
Send a POST request to /v1/chat/json:
```bash
curl -X POST http://localhost:3000/v1/chat/json \
  -d '{
    "model": "gpt-3.5-turbo",
    "messages": [{"role": "user", "content": "Say This is a test!"}]
}'
```
Send a POST request to /v1/stream_chat/json:
```bash
curl -X POST http://localhost:3000/v1/stream_chat/json \
  -d '{
    "model": "gpt-3.5-turbo",
    "messages": [{"role": "user", "content": "介绍一下Donald Trump"}]
}'
```
Send a POST request to /v1/embeddings:
```bash
curl -X POST http://localhost:3000/v1/embeddings \
  -d '{
    "input": "The food was delicious and the waiter...",
    "model": "text-embedding-ada-002"
}'
```
Send a POST request to /admin/create_user:
```bash
curl -X POST http://localhost:3000/admin/create_user \
  -H "Content-Type: application/json" \
  -d '{
  "request_count": 10
}'
```
Send a GET request to /admin/read_users:
```bash
curl http://localhost:3000/admin/read_users
```
Send a GET request to /admin/update_user:
```bash
curl http://localhost:3000/admin/update_user \
  -d '{
  "api_token":"sk-S8XbAUS6B14pvpiomaajtwWSuwS58EYw7POP9VNSffC4Ykx6",
  "delta": 10
}'
```
发送GET请求到/admin/delete_users：
```bash
curl http://localhost:3000/admin/delete_users \
  -H "Content-Type: application/json" \
  -d '{
  "api_tokens": ["sk-yGDpHLEzHErHPXjhpUk6CIBzdVYxUInUUQArHNt4aC925vCe", "sk-hlK5uPXWmKCJ4PjlY6QiQtnoKepvlJWsSdJJkL9VJWRLwkM7",
  "sk-URZMsGPnSoSapuliBxcOgjPd5tFDNQqP5nlk6Vdm52rlp8TJ"]
}'
## User
```
Send a POST request to /user/read_user:
```bash
curl http://localhost:3000/user/read_user \
  -d '{
  "api_token": "sk-S8XbAUS6B14pvpiomaajtwWSuwS58EYw7POP9VNSffC4Ykx6"
}'
```
Send a POST request to /user/decrement_user:
```bash
curl http://localhost:3000/user/decrement_user \
  -d '{
  "api_token": "sk-S8XbAUS6B14pvpiomaajtwWSuwS58EYw7POP9VNSffC4Ykx6"
}'
```

Replace http://localhost:3000 in the above commands with your actual server address and port. Please note that the POST requests use the -d option to specify an empty data body (""), as the example route handlers don't use the request body.

### Configuring the Embeddings Directory

To specify your own directory for embeddings data in the `src/models/storage/memory.rs` file, you need to replace `"wiki_5"` in `load_all_embeddings("wiki_5")` with your desired path.

Each file in your directory should be a JSON file with the following format:

```json
{
  "embedding": [
    -0.009146899,
    -0.021190763,
    ...
    -0.016805079
  ],
  "input": "Title: Bezeklik.... ",
  "total_tokens": 216,
  "uuid": "1234b15a-2b58-4be1-bab5-a810d06f4edc"
}
```

## Contributing
Contributions to GPTMiddleman are welcome. If you would like to contribute, please fork the repository and submit a pull request.

## License
GPTMiddleman is licensed under the MIT License.

## Downloaded Wiki Data
The wiki data used in this project was downloaded from [https://www.kaggle.com/datasets/stephanst/wikipedia-simple-openai-embeddings](https://www.kaggle.com/datasets/stephanst/wikipedia-simple-openai-embeddings).

