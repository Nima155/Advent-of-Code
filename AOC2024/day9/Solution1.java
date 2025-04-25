import java.io.File;
import java.io.IOException;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Deque;
import java.util.HashSet;
import java.util.List;
import java.util.Scanner;
import java.util.Set;


public class Solution1 {

    public static void main(String[] args) throws IOException {
        File f = new File("input.txt");

        Scanner scanner = new Scanner(f);


        
        StringBuffer sb = new StringBuffer();
        while (scanner.hasNextLine()) {
            sb.append(scanner.nextLine());
        }
        
        int arr[] = new int[sb.length()]; 
        Arrays.fill(arr, -1);
        int id = 0;

        for (int i = 0; i < sb.length(); i += 2) {
                arr[i] = id;
                id++;
              
        }
        
        List<Integer> ls = new ArrayList<>();
        
        for (int i = 0; i < sb.length(); i++) {
            int num = sb.charAt(i) - '0';
            for (int j = 0; j < num; j++) {
                if (i % 2 == 0) {
                    ls.add(arr[i]);
                } else if (i % 2 != 0) {
                    ls.add(-1);
                }
            }       
        }        
        
        // System.out.println(ls);
        Deque<Tuple> pq = new ArrayDeque<Tuple>();
        for (int i = 0; i < ls.size(); i++) {
            int tc = 0;
            int j = i;
            while (j < ls.size() && ls.get(j) == -1) {
                tc++;
                j++;
            }
            if (tc != 0) {
                pq.add(new Tuple(i, tc));
                i = j - 1;
            }
        }
        Set<Integer> visited = new HashSet<>();
        // System.out.println(ls);
        for (int j = ls.size() - 1; j >= 0; j--) {
            if (ls.get(j) != -1 && !visited.contains(ls.get(j))) {
                // System.out.println(ls.get(j));
                for (int i = 0; i < j; i++) {
                    int m = i;

                    while (m < ls.size() && ls.get(m) == -1) {
                        m++;
                    }

                    int targ = ls.get(j);
                    int z = j;
                    while (z >= 0 && ls.get(z) == targ) { z--; } 

                    int sL = m - i;
                    int sR = j - z;
                    if (sL >= sR) {
                        for (int v = i, w = 0; v < (i + sR); v++, w++) {
                            ls.set(v, targ);
                            ls.set(j - w, -1);
                        }
                    } 
                    visited.add(ls.get(j));
                }
                // System.out.println(ls);
            }
        }


        Long ans = 0L;
        for (int i = 0; i < ls.size(); i++) {
            if (ls.get(i) != -1) {
               ans += (long)ls.get(i) * i;
            }
            // System.out.println(new BigInteger(String.valueOf(ls.get(i))));

            
        }
        System.out.println(ls);
        System.out.println(ans);
    }
}