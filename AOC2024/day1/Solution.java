package AOC2024.day1;
import java.util.Comparator;
import java.util.LinkedList;
import java.util.List;
import java.io.File;  // Import the File class
import java.io.FileNotFoundException;  // Import this class to handle errors
import java.util.Scanner;
public class Solution {
    

    public static void main(String[] args) {
        try {
            File f = new File("input.txt");
            Scanner myReader = new Scanner(f);
            List<Integer> a = new LinkedList();
            List<Integer> b = new LinkedList();
            while (myReader.hasNextLine()) {
                String line = myReader.nextLine();
                String[] c = line.split("   ");
                
                a.add(Integer.valueOf(c[0]));
                b.add(Integer.valueOf(c[1]));
            }
            a.sort(Integer::compare);
            b.sort(Integer::compare);
            int c = 0;
            int sims = 0;
            int i = 0;
            for (; i < a.size();) {
                c += Math.abs(a.get(i) - b.get(i));
                int j = i;
                sims += a.get(i) * b.stream().filter(e-> e.equals(a.get(j))).count();
                i++;
            }
            System.out.println(sims);
            System.out.println(c);
        } catch (Exception e) {
            System.out.println("wawa!");
        }
    }
}