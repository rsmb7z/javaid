@
  Deprecated
@
SuppressWarnings	(	{ 	"a"
  , "b" 	,  } 	) package
demo
.
  annotations 	;  import
  java 	. lang
. 	annotation  . *	;
import
java
.
util .  List	;
@
Retention 	(	RetentionPolicy . RUNTIME	) @ 	Target
( {
  ElementType	.  TYPE_USE ,	ElementType
. METHOD
}
)	public  @ interface 	Annotations {
int
  value 	( 	)	default
  1  ; 	String name  ( )
default
"x"
; 	String [
]
names
(  ) default	{	"a"  ,
  "b" }
;
Class
<
  ?	>
  type ( 	)
  default	Object
  . class	;
Inner	inner  (
)  default  @
Inner ( a
= 	1  ,
b
=  {
2 , 3 } 	)
;
int
[ 	] 	nums 	(  )
  default
  { 	} ; int
  NUM
=
1
  ,
OTHER = 2
  ;
  String  s (
)
  [
  ] default  {
} 	; @
  interface	Inner	{ 	int a	(	) ;  int [ 	]
b (	) ;
}	enum
E
  {
  X } 	class
C	{  } interface
  I
{
} record R
( 	int a ) 	{
  } 	; 	}	@
  interface
Marker {
}
@ Annotations
(	value =
  1 ,
name = 	"n" , 	names
= {	} 	, type =  String  .
class ,	inner  =
@ Annotations
. Inner
( a
= 1 , b =
  2
)
)
class
  Use
  < @ Marker 	T
, @ Marker (
1
)  U extends  @	Marker  Number
  >	{
  @
Marker
int f ; 	@ Marker @  Deprecated (  since	=  "1"	)	@	SuppressWarnings
  (	"x"
)	public
void
  m  (	@	Marker  final
  int  a ,  @ 	Marker	String ...  rest )
{  }
List
  <	@	Marker	String 	>	l ;
List
<	@	Marker	?
extends
  @
  Marker  Number	> w	;
String
  @ Marker
[ 	] arr
; 	String
  @
Marker
[
] @ 	Marker [	] arr2
;	java
.	lang 	.	@
Marker	String  q	; 	Map	.
  @ Marker Entry
< String , String  >
  e
;
  int @	Marker [
] 	m2
(  )
  @
Marker
[  ] 	{ 	return 	null
; 	} 	void	body	( Object	o 	) throws
  @
Marker Exception 	{ @
Marker String
  s
  = (
@ 	Marker	String ) 	o	;
  Object  x	=  new
@  Marker
Object  ( )
  ; int	[ ]
  a
  =
  new
@	Marker
int 	@ Marker [ 3
  ]	;  int
  [
]
[ ]	b =
  new
  int  @
Marker
[  3  ]
  @
Marker [ 4	]
  ; for
( @ 	Marker  String	t :	new	String	[
  0
]
)
{
}
try	(
  @ Marker 	var  r  =
  open 	(  )  ) 	{ }	catch 	( @	Marker	RuntimeException
|
@ 	Marker Error 	e  )
{ } 	if
(
  o
instanceof  @
Marker	String
z
) {
} Runnable r
  =
(
  @ Marker
Runnable 	& java	.
  io  . Serializable 	) ( )	->
  { 	} ;
List  < String
  >
g
  = new
< 	String	>  ArrayList <
  String
>
(
)
  ; }  @
Marker
  @
Marker  <
  @	Marker
  V > 	@
  Marker	V  gen	(
  @ 	Marker
  V	v )
{ return
v
;
}
}
