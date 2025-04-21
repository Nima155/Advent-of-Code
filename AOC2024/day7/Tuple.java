public class Tuple implements Comparable<Tuple> {
    public int indx;
    public Long value;
    Tuple(int indx, long value) {
        this.indx = indx;
        this.value = value;
    }
    
    @Override
    public int compareTo(Tuple o) {
        if (this.indx != ((Tuple)o).indx) {
            return this.indx-((Tuple)o).indx;
        } return 0;
    }
}
