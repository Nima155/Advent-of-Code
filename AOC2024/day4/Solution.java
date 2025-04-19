import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;
import java.util.Vector;
public class Solution {

    public static int compareAndIncrement(StringBuffer sb) {
        if (sb.toString().equals("XMAS")) {
            // System.out.println(sb.toString());
            return 1;
        }
        return 0;
    }
    public static void main(String[] args) throws FileNotFoundException {
    

        File f = new File("input.txt");

        Scanner scanner = new Scanner(f);
        int tc = 0;
        Vector<int[]> vec = new Vector<>();
        while (scanner.hasNextLine()) {
            String line = scanner.nextLine();
            vec.add(line.chars().toArray());
        }
        
        for (int i = 0; i < vec.size(); i++) {
            for (int j = 0; j < vec.get(i).length; j++) {
                StringBuffer sb = new StringBuffer();
                for (int m = j; m < Math.min(vec.get(i).length, 4 + j); m++) { // checking forward
                    sb.append(Character.toString(vec.get(i)[m]));
                }
                tc += compareAndIncrement(sb);
                sb = new StringBuffer();
                for (int m = j; m >= Math.max(0, j - 3); m--) { // checking forward
                    sb.append(Character.toString(vec.get(i)[m]));
                }
                tc += compareAndIncrement(sb);
                sb = new StringBuffer();
                for (int m = i, v = j; m >= Math.max(0, i - 3) && v < Math.min(vec.get(m).length, j + 4); m--, v++) {
                    sb.append(Character.toString(vec.get(m)[v]));
                }
                tc += compareAndIncrement(sb);
                sb = new StringBuffer();
                for (int m = i, v = j; m >= Math.max(0, i - 3) && v >= Math.max(0, j - 4); m--, v--) {
                    sb.append(Character.toString(vec.get(m)[v]));
                }
                tc += compareAndIncrement(sb);
                sb = new StringBuffer();
                for (int m = i, v = j; m < Math.min(vec.size(), i + 4) && v >= Math.max(0, j - 3); m++, v--) {
                    sb.append(Character.toString(vec.get(m)[v]));
                }

                tc += compareAndIncrement(sb);
                sb = new StringBuffer();
                for (int m = i, v = j; m < Math.min(vec.size(), i + 4) && v < Math.min(vec.get(m).length, j + 4); m++, v++) {
                    sb.append(Character.toString(vec.get(m)[v]));
                }
                tc += compareAndIncrement(sb);
                sb = new StringBuffer();
                for (int m = i; m < Math.min(vec.size(), i + 4); m++) {
                    sb.append(Character.toString(vec.get(m)[j]));
                }
                tc += compareAndIncrement(sb);
                sb = new StringBuffer();
                for (int m = i; m >= Math.max(0, i - 3); m--) {
                    sb.append(Character.toString(vec.get(m)[j]));
                }
                tc += compareAndIncrement(sb);

            }
        }

        System.out.println(tc);
        scanner.close();;
        
    }
}