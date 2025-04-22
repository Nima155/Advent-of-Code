import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.PriorityQueue;
import java.util.Scanner;
import java.util.Set;
import java.util.stream.Stream;

public class Solution {

    public static void main(String[] args) throws FileNotFoundException {
        File f = new File("input.txt");

        Scanner scanner = new Scanner(f);


        
        StringBuffer sb = new StringBuffer();
        while (scanner.hasNextLine()) {
            sb.append(scanner.nextLine());
        }

        int arr[] = new int[sb.length()]; 
        // System.err.println(arr.length);
        Arrays.fill(arr, -1);
        int id = 0;
        // Stream.of(arr).forEach(System.out::println);
        for (int i = 0; i < sb.length(); i += 2) {
            // System.err.println(sb.charAt(i));
            if (arr[i] == -1) {
                arr[i] = id;
                id++;
            }  
        }
        // for (int i: arr) System.out.println(i);
        
        // Set<Integer> frees = new HashSet<>();
        List<Integer> ls = new ArrayList<>();
        PriorityQueue<Integer> pq = new PriorityQueue<>();

        for (int i = 0; i < sb.length(); i++) {
            int num = sb.charAt(i) - '0';
            for (int j = 0; j < num; j++) {
                if (i % 2 == 0) {
                    ls.add(arr[i]);
                } else if (i % 2 != 0) {
                    pq.add(ls.size());
                    ls.add(-1);
                }
            }       
        }        

        // Set<Integer> assigned = new HashSet<>();
        int j = ls.size() - 1;
        while (!pq.isEmpty() && j >= 0) {
            int indx = pq.peek();
            if (ls.get(j) != -1 && indx < j) {
                indx = pq.poll();
                ls.set(indx, ls.get(j));
                ls.set(j, -1);
                pq.add(j);
            }
            
            j--;
        }
        System.out.println(ls);
        Long ans = 0L;
        for (int i = 0; i < ls.size(); i++) {

            ans += ls.get(i) == -1 ? 0: ls.get(i) * i;
            
        }
        System.out.println(ans);
    }
}