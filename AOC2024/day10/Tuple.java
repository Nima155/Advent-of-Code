public class Tuple implements Comparable<Tuple> {
    public int indx;
    public int steps;
    Tuple(int indx, int value) {
        this.indx = indx;
        this.steps = value;
    }
    
    @Override
    public int compareTo(Tuple o) {
        if (this.steps != o.steps) {
            return this.indx - o.indx;
        }
        return o.noSlots - this.noSlots;
    }
}