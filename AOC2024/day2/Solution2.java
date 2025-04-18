import java.io.File;
import java.io.FileNotFoundException;
import java.util.Arrays;
import java.util.List;
import java.util.ArrayList;
import java.util.Scanner;
import java.util.stream.Collectors;

public class Solution2 {
    public static ArrayList<ArrayList<Integer>> allCombs(List<Integer> l) {
        ArrayList<ArrayList<Integer>> n = new ArrayList<>();


        for (int i = 0; i < l.size(); i++) {
            ArrayList<Integer> vec = new ArrayList<>();
            for (int j = 0; j < l.size(); j++) {
                if (j != i) {
                    vec.add(l.get(j));
                }
            }
            n.add(vec);
        }
        return n;
    }

    public static int handler(List<Integer> line) {
        boolean increasing = line.size() > 1 ? line.get(1) > line.get(0): true;
           
        for (int i = 0, j = 1; j < line.size(); j++, i++) {
            int diff = line.get(j) - line.get(i);
            if ((increasing == true && diff < 0) || (increasing == false && diff > 0) || Math.abs(diff) > 3 || Math.abs(diff) < 1) {
                return 0;
            }
        }
       
        return 1;
    }
    public static void main(String[] args) throws FileNotFoundException {
          File f = new File("input.txt");
        Scanner scanner = new Scanner(f);
        int safez = 0;  
        while (scanner.hasNextLine()) {
            List<Integer> line = Arrays.asList( scanner.nextLine().split(" "))
            .stream()
            .map(e-> Integer.valueOf(e))
            .collect(Collectors.toList());
            
            safez += allCombs(line).stream().anyMatch(e-> handler(e) == 1) == true ? 1: 0;

        }
        System.out.println(safez);
    }
}
