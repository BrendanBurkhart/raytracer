$fn = 100;

union() {
    difference() {
        cylinder(2, 1, 1, false);
        cylinder(2.1, 0.8, 0.8, false);
    }
    cylinder(0.2, 1, 1, false);
}
