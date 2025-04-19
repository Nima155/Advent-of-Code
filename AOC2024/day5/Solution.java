import java.io.File;
import java.io.FileNotFoundException;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Scanner;
import java.util.Set;
import java.util.Vector;

public class Solution {


    static int lastNow = -1;
    // private static boolean matcher(Map<Integer, Vector<Integer>> mp, ) 
    public static boolean comesAfter(int before, int now, Map<Integer, Vector<Integer>> map, Set<Integer> seto, int line[] ) {
        if (before == now) {
            return true;
        }
        boolean ret = false;
        for (int a: map.get(before)) { 
            if (!seto.contains(a) && Arrays.stream(line).filter(z-> z == a).count() == 1) {
                seto.add(a);
                ret |= comesAfter(a, now, map, seto, line);
            }
        }
        return ret;
    
    }

    public static void main(String[] args) throws FileNotFoundException {
        HashMap<Integer, Vector<Integer>> map = new HashMap<>();
        File f = new File("inpu.txt");
        Scanner scanner = new Scanner(f);
        boolean mode = false;
        int tc = 0;
        
        while (scanner.hasNextLine()) {
            String line = scanner.nextLine();
            if (mode && line.length() > 0) {
                int a[] = List.of(line.split(",")).stream().mapToInt(e-> Integer.valueOf(e)).toArray();
                boolean isValid = true;
                for (int i = 1; i < a.length; i++) {
                    // for (int j = 0; j < i; j++) {
                        if (!comesAfter(a[i-1], a[i], map, new HashSet<>(), a)) {
                            isValid = false;
                        // }
                    }
                    
                }
                if (isValid) {
                    tc += a[a.length / 2];
                }
            } else if (!mode && line.length() > 0) {
                String []s = line.split("\\|");
                // System.out.println(s[1]);
                if (!map.containsKey(Integer.valueOf(s[0]))) {
                    map.put(Integer.valueOf(s[0]), new Vector<Integer>(){ });
                } if (!map.containsKey(Integer.valueOf(s[1]))) {
                    map.put(Integer.valueOf(s[1]), new Vector<Integer>(){ });
                } 
                map.get(Integer.valueOf(s[0])).add(Integer.valueOf(s[1]));
            }

            if (line.length() == 0) {
                mode = true;
            }
            
            
        }
        System.out.println(tc);
        scanner.close();
    }
}