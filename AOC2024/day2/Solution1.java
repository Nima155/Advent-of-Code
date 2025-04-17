import java.io.File;
import java.io.FileNotFoundException;
import java.util.Arrays;
import java.util.List;
import java.util.Scanner;
import java.util.stream.Collectors;

public class Solution1 {

    public static void main(String[] args) throws FileNotFoundException {
        File f = new File("input.txt");
        Scanner scanner = new Scanner(f);
        int safez = 0;  
        while (scanner.hasNextLine()) {
            List<Integer> line = Arrays.asList( scanner.nextLine().split(" "))
            .stream()
            .map(e-> Integer.valueOf(e))
            .collect(Collectors.toList());
            boolean increasing = line.size() > 1 ? line.get(1) > line.get(0): true;
            boolean add = true;
           
            for (int i = 0, j = 1; j < line.size(); j++, i++) {
                int diff = line.get(j) - line.get(i);
                if ((increasing == true && diff < 0) || (increasing == false && diff > 0) || Math.abs(diff) > 3 || Math.abs(diff) < 1) {
                    add = false;
                    break;
                }
            }
            if (add) {
                safez++;
            }
        }
        System.out.println(safez);
    }
}