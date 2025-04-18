import java.util.regex.Pattern;
import java.util.stream.Stream;
import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;
import java.util.OptionalInt;
class Solution {
    public static void main(String[] args) throws FileNotFoundException {
        Pattern pat = Pattern.compile("mul\\(\\d+,\\d+\\)");
        Pattern pat2 = Pattern.compile("(?<=\\()\\d+,\\d+(?=\\))");
        File f = new File("input.txt");
        Scanner scanner = new Scanner(f);
        int res = 0; 
        while (scanner.hasNextLine()) {
            Stream<Stream<String[]>> s = pat.matcher(scanner.nextLine()).results().map(e-> 
                pat2.matcher(e.group()).results().map(j-> j.group().split(",")
            ));
            OptionalInt z = s.flatMap(e-> e)
                    .mapToInt(e-> Integer.valueOf(e[0]) * Integer.valueOf(e[1])).reduce((a, b)-> a + b);
            res += z.getAsInt();
        }
        System.out.println(res);
    }
}