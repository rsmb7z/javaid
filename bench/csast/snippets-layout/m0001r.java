package
demo ; import	static	java 	. lang .
Math . *
;
import	static 	java
.
util	. 	Collections	.
emptyList  ;
  import java
  .
util
. *	;
  import java	. 	util .  function
.  Function 	;  public
interface	Interfaces <
  T
extends  Comparable	<  ?
  super
  T
>
  &	java
.
io .
Serializable >
extends
Comparable
< T
  >
, 	Runnable { int	CONST = 1 	, 	OTHER [  ] 	=  {
  1 	,
  2  }
;
String NAME 	= "x"  ;
  void abstractMethod 	( ) throws 	java .	io . 	IOException ,
RuntimeException ;
default
  void 	def  ( )	{ }  static 	< U  >	U 	stat 	(	U 	u
)  { 	return
u ; 	} private void priv	(
)	{
} private
  static
int
  privStat 	(	)	{
return	1  ;
  }
< X  extends Exception > 	void
  generic  ( List <
  ? 	extends	X 	> a
,  List  < 	?
super X 	>
b
,
List	< ? 	>	c
)  throws X
;
  int arr
( 	)	[	] ; @ 	interface Ann  {
  int value
  ( ) 	default 	1	; String  [
  ]
names
(  ) 	default  { }
  ;  } class	Inner { } enum  E
{ A
  }
interface Nested {	}
record 	R (
int a ) {
} 	} abstract 	class	Abs	<	T
>
{
  abstract void
m 	( )	; protected  abstract
< 	U 	extends  T >
U n
( 	U u	)
; 	public 	static
final
synchronized
native void	nat 	( )
; transient 	volatile
int tv
; 	strictfp
  void	sf	( ) {  } 	public
<  T 	> Abs
(	)
{
}
  < T  > Abs
(
int
  x 	)  throws
  Exception
{
  }
} class
Outer {
class
In
  { In	( ) 	{ 	Outer
. this
  .
  hashCode (
)
  ; 	} void 	m ( ) 	{  Outer	. 	super
  .
toString	(
)
  ; }
  }	void m (
  )
{  Outer 	o
= new  Outer  (
)
  ;
Outer 	. 	In 	i 	=
  o	. 	new
In
  ( )  ;  class Local	{
}	interface
LI
{
}
  enum  LE	{	X }  record
LR (
int	a
) 	{	} final
  class
  FL
  {	}
  abstract 	class
AL {
} 	}
}  class	Sub extends 	Outer	.
  In
  {
Sub
  ( 	Outer o  ) {
  o
.
super ( )
  ;
}
Sub	( )	{  <  String >
super ( ) ;  }	Sub ( 	int  x 	)
  { super ( ) 	;	}
  Sub 	(	long x ) 	{
this 	(
)	;
}	Sub	( char
  x 	) { <	String
>
  this ( 	)	;
  }
}	