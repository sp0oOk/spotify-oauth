<?php

echo '<style>';
include "rainbow.css";
echo '</style>';

echo '<title>Spotify Verification</title>';

echo '<link rel="icon" href="icon.png" type="image/png" sizes="16x16">';

$url = "http://this-does-not-matter.com$_SERVER[REQUEST_URI]";

echo '<div class="wrapper"></div>';

echo '<div style="position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%);"><button onclick="copyToClipboard(\'' . $url . '\')">Click to copy</button></div>';

echo '<script>function copyToClipboard(text) {window.prompt("Copy to clipboard: Ctrl+C, Enter", text);}</script>';

?>