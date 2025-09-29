use std::time::Instant;
use std::time::Duration;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct Delay{
    when : Instant,
}

impl Future for Delay{
    type Output = String;

    fn poll(self: Pin<&mut Self> , cx : &mut Context<'_>) -> Poll<Self::Output> {
        
        if Instant::now() >=self.when{
            Poll::Ready(String::from("Here is your dosa"))
        }
        else{
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn main() {
   let _what_to_do = Delay{
    when : Instant::now() + Duration::from_secs(5),
   };
   println!("Waiting for dosa");
}
