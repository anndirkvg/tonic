pub mod pb {
    tonic::include_proto!("grpc.examples.echo");
}

use tokio_stream::{Stream, StreamExt};
use tonic::transport::Channel;
use std::time::Instant;

use pb::{echo_client::EchoClient, EchoRequest};

fn echo_requests_iter() -> impl Stream<Item = EchoRequest> {
    tokio_stream::iter(1..usize::MAX).map(|i| EchoRequest {
        message: format!("msg {:02}", i),
    })
}

async fn streaming_echo(client: &mut EchoClient<Channel>, num: usize) {
    let stream = client
        .server_streaming_echo(EchoRequest {
            message: "foo".into(),
        })
        .await
        .unwrap()
        .into_inner();

    // stream is infinite - take just 5 elements and then disconnect
    let mut stream = stream.take(num);
    let mut msg_count = 0;

    while let Some(_item) = stream.next().await {
        msg_count += 1;
    }
    // stream is dropped here and the disconnect info is send to server

    println!("\tstreaming_echo count: `{}`", msg_count);

}

async fn bidirectional_streaming_echo(client: &mut EchoClient<Channel>, num: usize) {
    let in_stream = echo_requests_iter().take(num);

    let response = client
        .bidirectional_streaming_echo(in_stream)
        .await
        .unwrap();

    let mut resp_stream = response.into_inner();
    let mut msg_count = 0;

    while let Some(_received) = resp_stream.next().await {
        msg_count += 1;
    }
    println!("\tbidirectional_streaming_echo count: `{}`", msg_count);

}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EchoClient::connect("http://[::1]:50051").await.unwrap();

    let num = 100;

    println!("Streaming echo:");
    let mut start = Instant::now();
    streaming_echo(&mut client, num).await;
    let mut duration = start.elapsed();
    println!("Time per call in streaming_echo() in ms: {:?}", duration.as_micros() as f32/((num as f32) * 1000.0));    

    // Echo stream that sends 17 requests then graceful end that connection
    println!("\r\nBidirectional stream echo:");
    start = Instant::now();    
    bidirectional_streaming_echo(&mut client, num).await;
    duration = start.elapsed();    
    println!("Time per call in bidirectional_streaming_echo() in ms: {:?}", duration.as_micros() as f32/((num as f32) * 1000.0));    

    Ok(())
}
