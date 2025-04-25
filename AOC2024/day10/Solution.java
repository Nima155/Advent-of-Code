import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Deque;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Scanner;
import java.util.Set;
import java.util.stream.Collectors;

public class Solution {
    private static final int mov[][] = { {1, 0}, {0, 1}, {-1, 0}, {0, -1}};
    public static int traverser(int sti, int stj, List<int[]> mat) {
        Set<String> visited = new HashSet<>();
        Deque<int[]> dq = new ArrayDeque<int[]>();
        int sm = 0;
        dq.add(new int[]{sti, stj, 0});
        while (!dq.isEmpty()) {
            int[] fr = dq.pollFirst();
            for (int[]mv: mov) {
                int nI = mv[0] + fr[0];
                int nJ = mv[1] + fr[1];
                String k = nI + " " + nJ;
                if (!visited.contains(k) && nI >= 0 && nI < mat.size() && nJ >= 0 && nJ < mat.get(0).length && mat.get(nI)[nJ] == (fr[2] + 1)) {
                    if(mat.get(nI)[nJ] == 9) sm++;
                    visited.add(k);
                    dq.addLast(new int[]{nI, nJ, fr[2] + 1});
                }
            }

        }
        return sm;
    }
    public static void main(String[] args) throws FileNotFoundException {
        File f = new File("test.txt");

        Scanner scanner = new Scanner(f);


        
        ArrayList<int[]> matrix = new ArrayList<>();
        while (scanner.hasNextLine()) {
            int[] arr = scanner.nextLine().chars().map(e-> (e - '0')).toArray();
            matrix.add(arr);
        }

        int sm = 0;
        for (int i = 0; i < matrix.size(); i++) {
            for (int j = 0; j < matrix.get(i).length; j++) {
                if (matrix.get(i)[j] == 0) {
                    sm += traverser(i, j, matrix);
                }
            }
        }

        System.out.println(sm);

    }
}