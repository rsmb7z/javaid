import java.util.List;

class RcMsgProbe {
	static class AnnotatedField {

		@Deprecated
		String text1;

		@Deprecated
		String text2; }

	static class PlainField {
		String text1;
		String text2; }

	enum WithBody {
		A { void f() { } };

		abstract void f(); }

	enum NoMembers {
		A, B; }

	enum Constants {
		A, B }

	enum Trailing {
		A, B, }

	interface Constant {
		int X = 1; }

	static class AbstractMethod {
		abstract void arrange(List<Integer> list); }

	static class Mixed {
		void m() {
			int a = 1;
			int b = 2; }
		void n() {
			foo(); }
		void foo() { return; }
		void o() {
			if (true) { foo(); } else { foo(); }
			try { foo(); } catch (RuntimeException e) { foo(); } finally { foo(); }
			do { foo(); } while (false);
			while (true) { foo(); break; }
			for (;;) { foo(); break; }
			Runnable r = new Runnable() { public void run() { foo(); } };
			synchronized (this) { foo(); }
		}
		String ü = "é"; void p() {
			foo(); } String ï; }
}
