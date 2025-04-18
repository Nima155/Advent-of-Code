import java.util.regex.Pattern;
import java.util.stream.Collector;
import java.util.stream.Stream;
import java.io.File;
import java.io.FileNotFoundException;
import java.lang.reflect.Array;
import java.util.Scanner;
import java.util.OptionalInt;
class Solution2 {
    public static void main(String[] args) throws FileNotFoundException {
        Pattern pat = Pattern.compile("mul\\(\\d+,\\d+\\)|do(n't)?\\(\\)");
        Pattern pat2 = Pattern.compile("(?<=\\()\\d+,\\d+(?=\\))");
        File f = new File("input.txt");
        Scanner scanner = new Scanner(f);
        Long res = 0L; 
        boolean dz = true;
        while (scanner.hasNextLine()) {
            String[] ret = pat.matcher(scanner.nextLine()).results().map(e-> 
               e.group()
            ).toArray(z-> new String[z]);
            for (String  mat: ret) {
                // System.out.println(mat);    
                if (mat.equals("do()")) {
                    dz = true; 
                } else if (mat.equals("don't()")) {
                    dz = false;
                }
                else if (dz == true) {
                    System.out.println(mat + " " + res);
                    res += pat2.matcher(mat).results().map(j-> j.group().split(","))
                    .mapToLong(e-> Integer.valueOf(e[0]) * Integer.valueOf(e[1])).reduce((a, b) -> a + b).getAsLong();
                    // System.out.println(mat + " " + res);
                }
            }
            
            // s.flatMap(e-> e).forEach(j -> {
            // });;
        }
        System.out.println(res);
    }
}