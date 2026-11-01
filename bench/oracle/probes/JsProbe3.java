package probes;

import java.util.List;

public interface JsProbe3 {
    /**
     * Generic, no modifiers.
     * @return the value
     * @param x the arg
     */
    <T> T first(List<T> x);

    /**
     * Generic with public.
     * @return the value
     * @param x the arg
     */
    public <T> T second(List<T> x);

    /**
     * Plain.
     * @return the value
     * @param x the arg
     */
    String third(String x);
}
