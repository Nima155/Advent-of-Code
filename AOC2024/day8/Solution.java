import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Scanner;
import java.util.Set;


public class Solution {
    public static void positionFinder(int a[], Set<String> positions, List<char[]> ls) {
        int iUp = a[0] + a[0] - a[2];
        int jUp = a[1] + a[1] - a[3];

        int i2Down = a[2] + a[2] - a[0];
        int j2Down = a[3] + a[3] - a[1];

        if (iUp >= 0 && jUp >= 0 && jUp < ls.get(0).length ) {
            positions.add(iUp + " " + jUp);
        }
        if (i2Down < ls.size() && j2Down >= 0 && j2Down < ls.get(0).length) {
            positions.add(i2Down+ " " + j2Down);
        }
        
    }

    public static void main(String[] args) throws FileNotFoundException {
            File file = new File("input.txt");
            Scanner scanner = new Scanner(file);
            ArrayList<char[]> ls = new ArrayList<>();
            while (scanner.hasNextLine()) {
                ls.add(scanner.nextLine().toCharArray());
            }

            Map<Integer, List<String>> mp = new HashMap<>();

            for (int i = 0; i < ls.size(); i++) {
                for (int j = 0; j < ls.get(i).length; j++) {
                    int chara = ls.get(i)[j];
                    if (chara != '.') {
                        if (!mp.containsKey(chara)) {
                            mp.put(chara, new ArrayList<>());
                        } 
                        mp.get(chara).add(i + " " + j);
                    }       
                }
            }

            Set<String> visited = new HashSet<>();
            for (int a: mp.keySet()) {
                for (int m = 0; m < mp.get(a).size(); m++) {

                    var pos = mp.get(a).get(m).split(" ");
                    int i = Integer.valueOf(pos[0]);
                    int j = Integer.valueOf(pos[1]);

                    for (int w = m + 1; w < mp.get(a).size(); w++) {                        
                        var pos1 = mp.get(a).get(w).split(" ");
                        int i2 = Integer.valueOf(pos1[0]);
                        int j2 = Integer.valueOf(pos1[1]);
                        int z[] = { i, j, i2, j2};
                        positionFinder(z, visited, ls);

                    }

                }
            }

            // System.err.println(visited);

            System.err.println(visited.size()); 








    }
}