<?php

while (($line = fgets(STDIN)) !== false) {
    $matches = [];
    preg_match('/\d+$/', $line, $matches);

    if (!isset($matches[0])) {
        continue;
    }

    $url_number = $matches[0];

    echo $url_number."\t".'1'."\n";
}