class Count {
    static void count() {
        float x = 0.0f;
        while (x != x + 1.0f) {
            x += 1.0f;
        }
        System.out.println(x);
    }

    public static void main(String[] args) {
        count();
    }
}