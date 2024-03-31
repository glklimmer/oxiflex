var 1..4: w;
var 1..4: y;
var 1..3: x;
var 1..3: z;

constraint w = 2*x;
constraint w < z;
constraint y > z;

solve satisfy;

