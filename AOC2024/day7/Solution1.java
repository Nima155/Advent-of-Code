import java.io.File;
import java.io.FileNotFoundException;
import java.util.Arrays;
import java.util.PriorityQueue;
import java.util.Scanner;
// 265906490178
class Solution1 {
    public static boolean checker(long a[], long res, long goal) {
        PriorityQueue<Tuple> pq = new PriorityQueue<>();
        pq.add(new Tuple(0, a[0]));
        for (int i = 1; i < a.length; i++) {
            PriorityQueue<Tuple> pq2 = new PriorityQueue<>();
            while (!pq.isEmpty()) {
                Tuple p1 = pq.poll();
                pq2.add(new Tuple(i, p1.value * a[i]));
                pq2.add(new Tuple(i, p1.value + a[i]));
                // pq2.add(new Tuple(i, Long.valueOf(String.valueOf(p1.value) + String.valueOf(a[i])))); % uncomment for part 2
            }
            pq = pq2;
        }
        while (!pq.isEmpty()) {
            Tuple p1 = pq.poll();
            if (p1.value == goal) {
                return true;
            }
        }
        return false;
    }

    public static void main(String[] args) throws FileNotFoundException {
        File file = new File("input.txt");
        Scanner scanner = new Scanner(file);
        Long tc = 0L;
        while (scanner.hasNextLine()) {
            String[] line = scanner.nextLine().split(": ");
            long []b = Arrays.stream(line[1].split(" "))
            .mapToLong(e-> Long.valueOf(e))
            .toArray();
            if (checker(b, b[0], Long.valueOf(line[0]))) {
                tc += Long.valueOf(line[0]);
            }
        }

        System.out.println(tc);
        scanner.close();
    }
}