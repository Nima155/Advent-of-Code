public class Tuple implements Comparable<Tuple> {
    public int indx;
    public int noSlots;
    Tuple(int indx, int value) {
        this.indx = indx;
        this.noSlots = value;
    }
    
    @Override
    public int compareTo(Tuple o) {
        if (this.indx != o.indx) {
            return this.indx - o.indx;
        }
        return o.noSlots - this.noSlots;
    }
}