import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Scanner;

public class Solution1 {
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
    public static int z = 0;
    public static Long recurser(String starter, Map<String, Long> memo, int c) {
        Long ans = 1l;
        // System.out.println(starter);

        if (c == 0) {   
            return ans;
        }

        String k = starter + " c:" + c;
        if (memo.containsKey(k)) {
            return memo.get(k);
        }
        if (starter.equals("0")) {
            return recurser("1", memo, c - 1);
        }
        if (starter.length() % 2 == 0) {
            ans = 0l;
            // System.out.println(starter.subSequence(0, starter.length() / 2).toString());
            // System.out.println(starter.subSequence(starter.length() / 2, starter.length()).toString());
            ans += (recurser(removeLeading(starter
            .subSequence(0, starter.length() / 2).toString()), memo, c - 1));

            ans += (recurser(removeLeading(starter
            .subSequence(starter.length() / 2, starter.length()).toString()),  memo, c - 1));
        } else {
            ans = 0l;
            ans += recurser(String.valueOf(Long.valueOf(starter) * 2024), memo, c - 1);
        }
        memo.put(k, ans);
        return ans;
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
        Map<String, Long> mp = new HashMap<>();
        Long ans = 0l;
        for (int j = 0; j < matrix.size(); j++) {
            if (matrix.get(j).equals("0")) {
                ans += recurser(matrix.get(j), mp, 75);
            } else {
                ans += recurser(matrix.get(j), mp, 75);
            }
            
        }
        System.out.println(ans);

    }
}
