/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/
use std::clone::Clone;

#[derive(Debug)]
pub struct Queue<T: Clone> {
    elements: Vec<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

	pub fn clone(&self) -> Self {
		let mut res = vec![];
		for ele in &self.elements {
			res.push(ele.clone());
		}
		Queue {
			elements: res
		}
	}
}

impl<T: Clone> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T: Clone>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>,
	size: usize
}
impl<T: Clone> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			size: 0,
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
		self.q2.enqueue(elem);
		while !self.q1.is_empty() {
			self.q2.enqueue(self.q1.dequeue().unwrap());
		}
		self.size += 1;
		// let temp = self.q2.clone();
		// self.q2 = self.q1.clone();
		// self.q1 = temp.clone();
        std::mem::swap(&mut self.q1, &mut self.q2);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
		if self.size == 0 {
			return Err("Stack is empty")
		}
		self.size -= 1;
		self.q1.dequeue()
    }
    pub fn is_empty(&self) -> bool {
		//TODO
		self.size == 0
    }
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}