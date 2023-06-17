// src/middleware/utils/messages_process.rs
// 接收一个String和String，返回一个String，其中要对两个String进行处理。
pub fn process_messages(question: &str, context: &str) -> String {
    let formatted_context = format!("{}\n", context);
    let formatted_question = format!("Question: {}\n", question);
    // let text = format!(
    //     "You are a helpful AI assistant. Use the following pieces of context to answer the question at the end.\nPlease provide your response in Chinese (Simplified). If you don't know the answer, just say you don't know. DO NOT try to make up an answer.\nIf the question is not related to the context, politely respond that you are tuned to only answer questions that are related to the context.\n\n{}\n\n{}Helpful answer in markdown:",
    //     formatted_context, formatted_question
    // );

    let text = format!(
        "You are a helpful AI assistant. Use the following pieces of context to answer the question at the end.\nPlease provide your response in Chinese (Simplified). \n\n{}\n\n{}Helpful answer in markdown:",
        formatted_context, formatted_question
    );

    text
}
