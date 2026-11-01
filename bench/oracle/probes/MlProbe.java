package probes;

/* single line */
/*
 * proper
 */
/*
   missing one
 * proper
 */
/*
 first
 second
 * third
 */
/* text
 * more
   */
/* text
 more */
/**/
/***/
/*
*/
/* a
b
*/
/*

 */
/**
 no star javadoc attached
 */
public class MlProbe {
    /**
     no star javadoc misplaced
     */
    /* between
       no star */
    void a() {
        /*
         no star in body
         * star
        */
        int x = /* inline
          multi */ 1;
        foo(/* a
        b */);
    }

    /**
     * Wrongly placed javadoc
     none here
     */
    int y;

    void foo(Object o) {}

	/*
	 * tab
	   spaced
	 */
    /*
    	 weird
    */
    /* crlf-free */
    void z() {
        /**
          javadoc in body
         */
        int q = 1;
    }
}
/**
 trailing javadoc at file end
 */
