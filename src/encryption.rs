/*
Algoritmos de Cifrado en el Ransomware Moderno
    El ransomware moderno utiliza una técnica muy específica y eficaz conocida como cifrado híbrido. Este método combina lo mejor de dos mundos: la velocidad del cifrado simétrico y la seguridad del cifrado asimétrico.

Cifrado Simétrico para los archivos: 
    El ransomware utiliza un algoritmo de cifrado simétrico, como AES, para cifrar los archivos de la víctima. ¿Por qué AES? Porque es increíblemente rápido y eficiente para cifrar grandes volúmenes de datos. Se genera una clave simétrica única para cada archivo (o para un grupo de archivos).

Cifrado Asimétrico para la clave simétrica: 
    Una vez que los archivos están cifrados, el ransomware utiliza un algoritmo de cifrado asimétrico, como RSA o ECC, para cifrar la clave simétrica. El ransomware genera un par de claves asimétricas: una pública y una privada. La clave pública se usa para cifrar la clave simétrica y se deja en el sistema de la víctima. La clave privada, que es la única que puede descifrar la clave simétrica, se envía y se almacena en el servidor de comando y control (C&C) de los atacantes.

Esta estrategia es altamente efectiva porque:

Velocidad: El cifrado de los archivos se realiza rápidamente con AES, minimizando el tiempo que los atacantes necesitan para completar el proceso.

Seguridad: La clave privada que se necesita para la recuperación nunca sale del control de los atacantes. Sin esta clave, es prácticamente imposible descifrar los datos, haciendo que el rescate sea la única opción viable para la víctima.

Eficiencia: El único dato que necesita ser enviado al servidor de los atacantes es la clave simétrica cifrada, que es muy pequeña, lo que reduce el tráfico de red y la posibilidad de ser detectado.

Ejemplos de ransomware que utilizan esta técnica incluyen a grupos notorios como LockBit, Conti, y BlackCat, que han adoptado esta metodología para maximizar la eficacia de sus ataques. Algunos de los más recientes como VanHelsing están usando algoritmos como Curve25519 (para el intercambio de claves) y ChaCha20 (para el cifrado de archivos), mostrando una diversificación en las herramientas utilizadas por los ciberdelincuentes.
*/