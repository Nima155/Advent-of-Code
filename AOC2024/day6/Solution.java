import java.io.File;
import java.io.FileNotFoundException;
import java.util.HashSet;

import java.util.List;
import java.util.Scanner;
import java.util.Set;
import java.util.Vector;


public class Solution {

    public static void main(String[] args) throws FileNotFoundException {
        File f = new File("input.txt");
        Scanner scanner = new Scanner(f);
        Set<String> visited = new HashSet<>();
        List<String> nb = new Vector<>();
        while (scanner.hasNextLine()) {
            String line = scanner.nextLine();
            nb.add(line);
        }
        scanner.close();

        int r = 0, c = 0;
        for (int i = 0; i < nb.size(); i++) {
            for (int j = 0; j < nb.get(i).length(); j++) {
                if (nb.get(i).charAt(j) == '^') {
                    r = i;
                    c = j; 
                    nb.set(i, nb.get(i).replace('^', '.'));
                }
            }
        }
        
        visited.add(String.valueOf(r) +" "+ String.valueOf(c));
        do {
            while ((r-1) >= 0 && nb.get(r-1).charAt(c) == '.' ) {
                r--; 
                visited.add(String.valueOf(r) +" "+ String.valueOf(c)); 
            }
            if ((r-1) < 0) { break; }

            while ((c+1) < nb.get(r).length() && nb.get(r).charAt(c+1) == '.') {
                c++; 
                visited.add(String.valueOf(r) +" "+ String.valueOf(c)); 
            }
            if ((c + 1) == nb.get(r).length()) { break; }

            
            while ((r+1) < nb.size() && nb.get(r+1).charAt(c) == '.') {
                r++; 
                visited.add(String.valueOf(r) +" "+ String.valueOf(c)); 
            }
            if ((r+1) == nb.size()) { break; }


            
            while ((c-1) >= 0 && nb.get(r).charAt(c-1) == '.') {
                c--; 
                visited.add(String.valueOf(r) +" "+ String.valueOf(c)); 
            }
            if ((c-1) < 0) { break; }

            
        } while (true);
        
        System.out.println(visited.size());

        
    }
}