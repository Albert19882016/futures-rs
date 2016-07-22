use std::io;

use {Poll, Task};
use stream::Stream;
use io::ReadStream;

pub struct Chain<A, B> {
    a: A,
    b: B,
    first: bool,
}

pub fn new<A, B>(a: A, b: B) -> Chain<A, B>
    where A: ReadStream,
          B: ReadStream,
{
    Chain {
        a: a,
        b: b,
        first: true,
    }
}

impl<A, B> Stream for Chain<A, B>
    where A: ReadStream,
          B: ReadStream,
{
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self, task: &mut Task) -> Poll<Option<()>, io::Error> {
        if self.first {
            self.a.poll(task)
        } else {
            self.b.poll(task)
        }
    }

    fn schedule(&mut self, task: &mut Task) {
        if self.first {
            self.a.schedule(task)
        } else {
            self.b.schedule(task)
        }
    }
}

impl<A, B> ReadStream for Chain<A, B>
    where A: ReadStream,
          B: ReadStream,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<Option<usize>> {
        if self.first {
            match self.a.read(buf) {
                Ok(Some(0)) => self.first = false,
                other => return other,
            }
        }
        self.b.read(buf)
    }
}
