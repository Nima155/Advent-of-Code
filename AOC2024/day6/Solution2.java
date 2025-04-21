import java.io.File;
import java.io.FileNotFoundException;
import java.util.HashSet;

import java.util.List;
import java.util.Scanner;
import java.util.Set;
import java.util.Vector;


public class Solution2 {

    public static void main(String[] args) throws FileNotFoundException {
        File f = new File("input.txt");
        Scanner scanner = new Scanner(f);
        Set<String> visited = new HashSet<>();
        List<StringBuffer> nb = new Vector<>();
        while (scanner.hasNextLine()) {
            StringBuffer line = new StringBuffer(scanner.nextLine());
            nb.add(line);
        }
        scanner.close();

        int r = 0, c = 0;
        for (int i = 0; i < nb.size(); i++) {
            for (int j = 0; j < nb.get(i).length(); j++) {
                if (nb.get(i).charAt(j) == '^') {
                    r = i;
                    c = j; 
                    nb.set(i, nb.get(i).replace(j, j+1, "."));
                }
            }
        }
        
        
        for (int i = 0; i < nb.size(); i++) {
            for (int j = 0; j < nb.get(i).length(); j++) {
                if (nb.get(i).charAt(j) != '#') {
                    Set<String> visited2 = new HashSet<>();
                    nb.get(i).replace(j, j+1, "#");
                    if (isPossible(r, c, nb, visited2) == 1) {
                        visited.add(String.valueOf(i) +" "+ String.valueOf(j));
                    }
                    nb.get(i).replace(j, j + 1, ".");
                }
            }
        }
        visited.remove(String.valueOf(r) +" "+ String.valueOf(c));
        System.out.println(visited.size());

    }

    public static int isPossible(int r, int c, List<StringBuffer> nb, Set<String> visited) {
        int loop = 0;
        do {
            while ((r-1) >= 0 && nb.get(r-1).charAt(c) == '.' ) {
                r--; 
 
            }
            if ((r-1) < 0) { break; }

            while ((c+1) < nb.get(r).length() && nb.get(r).charAt(c+1) == '.') {
                c++; 
            }
            if ((c + 1) == nb.get(r).length()) { break; }
            
            while ((r+1) < nb.size() && nb.get(r+1).charAt(c) == '.') {
                r++; 
            }
            if ((r+1) == nb.size()) { break; }

            while ((c-1) >= 0 && nb.get(r).charAt(c-1) == '.') {
                c--; 
            }
            if ((c-1) < 0) { break; }
            loop++;
        } while (loop < 100);
        return loop == 100 ? 1: 0;
    }
    
}
