import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;
import java.util.Vector;
public class Solution1 {

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
                if (i > 0 && i < (vec.size()-1) && j > 0 && j < (vec.get(i).length-1)) {
                    if (vec.get(i)[j] == 'A' && vec.get(i+1)[j+1] == 'M' 
                    && vec.get(i+1)[j-1] == 'M' && vec.get(i-1)[j+1] == 'S' && vec.get(i-1)[j-1] == 'S'){
                        tc++;
                    }

                    if (vec.get(i)[j] == 'A' && vec.get(i+1)[j+1] == 'S' 
                    && vec.get(i+1)[j-1] == 'S' && vec.get(i-1)[j+1] == 'M' && vec.get(i-1)[j-1] == 'M'){
                        tc++;
                    }

                    if (vec.get(i)[j] == 'A' && vec.get(i+1)[j+1] == 'S' 
                    && vec.get(i+1)[j-1] == 'M' && vec.get(i-1)[j+1] == 'S' && vec.get(i-1)[j-1] == 'M'){
                        tc++;
                    }

                    if (vec.get(i)[j] == 'A' && vec.get(i+1)[j+1] == 'M' 
                    && vec.get(i+1)[j-1] == 'S' && vec.get(i-1)[j+1] == 'M' && vec.get(i-1)[j-1] == 'S'){
                        tc++;
                    }
                }
            }
        }

        System.out.println(tc);
        scanner.close();;
        
    }
}