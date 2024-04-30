
pub struct Queue {
    _date: String,
    arr: Vec<u16>
}

impl Queue {
    pub fn new() -> Queue {
        Queue { 
            _date: "04/21/24".to_string(),
            arr: Vec::new()
        }
    }

    pub fn enqueue(&mut self, item: u16){
        self.arr.push(item);
        if self.arr.len() > 2 {
            let mut ind = self.arr.len() -1;
            while self.arr[ind] < self.arr[ind/2] {
                if self.arr[ind/2] >=1 {
                    (self.arr[ind], self.arr[ind/2]) = (self.arr[ind/2], self.arr[ind]);
                    if ind / 2 > 1 {
                        ind = ind / 2;
                    }
                    else {break;}
                }
            }
        }
    }

    pub fn dequeue(&mut self) -> u16{
        if self.arr.len() <= 1{
            return 0;
        }

        let root = self.arr[1];
        self.arr[1] = self.arr[self.arr.len()-1];
        self.arr.remove(self.arr.len()-1);

        if self.arr.len() >2 {
            if self.arr.len() == 3 {
                (self.arr[1], self.arr[2]) = (self.arr[2], self.arr[1]);

                return root;
            }

            let mut i = 1;
            let mut l = 2 * i;
            let mut r = 2 * i + 1;
            while self.arr[i] >= self.arr[l] || self.arr[i] >= self.arr[r] {
                if self.arr[l] < self.arr[r] {
                    (self.arr[i], self.arr[l]) = (self.arr[l], self.arr[i]);
                    i = i*2;
                }
                else {
                    (self.arr[i], self.arr[l]) = (self.arr[l], self.arr[i]);
                    i = i*2+1;
                }

                l=2*i;
                r = 2*i+1;
                if l >= self.arr.len() || r >= self.arr.len() {
                    break;
                }
            }
        }
        else if self.arr.len() <=2 {
            return root;
        }

        return root;
    }

    fn sort(&self){
        // let newArr: Vec<u16> = Vec::new();
        // while &self.arr.len() >= 1{
        //     newArr.append(dequeue());
        // }
    }

    pub fn view(&mut self){
        for item in self.arr.as_slice() {
            print!("{}",item)
        }
    }
}