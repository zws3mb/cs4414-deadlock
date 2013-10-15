use std::task;
fn main(){
let(port,chan): (Port<int>, Chan<int>) = stream();//set up two streams

let(port2,chan2): (Port<int>, Chan<int>) = stream();

//do
task:: spawn_sched(task::SingleThreaded, || {//task 1
println("Task 1");
/*let mut b =1;
while(b<1000000){
b=b+2;
}
println(fmt!("1: %i",b));*/
let result = 4+port2.recv();//wait for input from task 2
chan.send(result);
println("Never finished 1");
});

//do {
task::spawn_sched(task::SingleThreaded, || {
println("Task 2");
/*let mut i =0;
while(i<100){
println(fmt!("2: %i",i));
i=i+1;
}*/
let deadresult = port.recv();//wait for input from task 1
chan2.send(deadresult);
println("Never finished 2");
});//};
//Hopefully this is a sufficient deadlock--two threads of the same priority waiting on each other. A more sophisticated deadlock might include priority inversion or work on something more than a simple i/o circular reasoning.

}
