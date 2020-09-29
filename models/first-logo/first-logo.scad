$fn=200;

rotate([30, 10, 0]) {
    difference() {
        color("white") cylinder(1.5,6,6,true);
        cylinder(2,4.5,4.5,true);
    }
}

translate([6, 1, 0]) {
    rotate([-20,0,0]) rotate([0,45,0]) rotate([0,0,0]) {
        difference() {
            color("blue") cube([10, 1.5, 10], true);
            cube([7, 2, 7], true);
        }
    }
}

translate([-6, -2, 0]) {
rotate([80, 0, 30]) {
linear_extrude(height=1.5, scale=[1,1], slices=1, twist=0)
difference() {
    color("red") polygon( points=[[-7,-tan(30)*7],[7,-tan(30)*7],[0,7*sqrt(3)-tan(30)*7]]);
    polygon( points=[[-4.4,-tan(30)*4.4],[4.4,-tan(30)*4.4],[0,4.4*sqrt(3)-tan(30)*4.4]] );
}}}