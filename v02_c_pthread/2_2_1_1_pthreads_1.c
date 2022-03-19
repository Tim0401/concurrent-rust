#include <pthread.h> // <1>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#define NUM_THREADS 10 // 生成するスレッドの数

// スレッド用関数
void* thread_func(void *arg) { // <2>
    int id = (int)arg; // <3>
    for (int i = 0; i < 5; i++) { // <4>
        printf("id = %d, i = %d\n", id, i);
        sleep(id);
    }

    char* buffer = (char*)malloc(sizeof(char) * 255);
    snprintf(buffer, sizeof(char) * 255, "finished! id = %d", id);
    return buffer;
}

int main(int argc, char *argv[]) {
    pthread_t v[NUM_THREADS]; // <5>
    // スレッド生成 <6>
    for (int i = 0; i < NUM_THREADS; i++) {
        if (pthread_create(&v[i], NULL, thread_func, (void *)i) != 0) {
            perror("pthread_create");
            return -1;
        }
    }

    // スレッドの終了を待機 <7>
    for (int i = 0; i < NUM_THREADS; i++) {
        char *ptr;
        if (pthread_join(v[i], (void **)&ptr) == 0) {
            printf("msg = %s\n", ptr);
            free(ptr);
        } else {
            perror("pthread_join");
            return -1;
        }
    }

    return 0;
}