package demo;

import java.util.*;

public class Demo {
  private int Count;

  public void do_work(List<String> items) {
    if (items.isEmpty()) return;
    for (String s : items) {
        System.out.println(s);
    }
  }
}
