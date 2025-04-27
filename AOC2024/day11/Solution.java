import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Scanner;

public class Solution {
    public static String removeLeading(String sb) {
        if (sb.startsWith("0") && sb.length() == 1) {
            return sb;
        }
        StringBuffer sbz = new StringBuffer();
        int i = 0;
        while (i < sb.length() &&  sb.charAt(i) == '0') i++;
        if (i == sb.length()) return "0";
        for (; i < sb.length(); i++) {
            sbz.append(sb.charAt(i));
        }
        return sbz.toString();
    }
    public static void main(String[] args) throws FileNotFoundException {
        File f = new File("input.txt");

        Scanner scanner = new Scanner(f);


        
        List<String> matrix = new ArrayList<>();
        while (scanner.hasNextLine()) {
            for (String num: scanner.nextLine().split(" "))  {
                matrix.add(num);
            }
            // matrix.add(arr);
        }
        Map<String,String> mp = new HashMap<>();
        List<String> prelim = new ArrayList<>();
        for (int i = 0; i < 25;i++) {
            for (int j = 0; j < matrix.size(); j++) {
                if (matrix.get(j).equals("0")) {
                    prelim.add("1");
                } else if (matrix.get(j).length() % 2 == 0) {
                    StringBuffer sb = new StringBuffer();
                    StringBuffer sb2 = new StringBuffer();
                    for (int v = 0, m = matrix.get(j).length()/2; v < matrix.get(j).length()/2; v++, m++) {
                        // System.out.println(111 + " " + matrix.get(j).length() + " " + v + " " + m);
                        sb.append(matrix.get(j).charAt(v));
                        sb2.append(matrix.get(j).charAt(m));
                    }
                    // System.out.println(111 + " ");
                    String sbb = removeLeading(sb.toString());
                    String sbb2 = removeLeading(sb2.toString());
                    // System.out.println(sbb2);/
                    
                    prelim.add(sbb);
                    prelim.add(sbb2);
                } else {
                    // System.out.println(matrix.get(j));
                    // System.out.println(Integer.valueOf(matrix.get(j)) * 2024);
                    prelim.add(String.valueOf(Long.valueOf(matrix.get(j)) * 2024));
                }
            }
            

            matrix = prelim;
            prelim = new ArrayList<>();
            // System.out.println(matrix);
            // if (i==2) break;
        }
        System.out.println(matrix.size());

        // System.out.println(sm);

    }
}
