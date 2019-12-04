#!/bin/sh
# See <https://github.com/nanoc/nanoc/issues/1473>.
set -eu

cd "$(dirname "$(readlink -f "$0")")"

RACK_LIVERELOAD_PATH="$(realpath --relative-to=. "$(bundle show rack-livereload)")"
echo "rack-livereload: ${RACK_LIVERELOAD_PATH}"

patch -p0 <<__EOF__
diff -Naur ${RACK_LIVERELOAD_PATH}/lib/rack/livereload/processing_skip_analyzer.rb rack-livereload-0.3.17/lib/rack/livereload/processing_skip_analyzer.rb
--- ${RACK_LIVERELOAD_PATH}/lib/rack/livereload/processing_skip_analyzer.rb 2019-11-29 19:20:38.213601705 +0900
+++ ${RACK_LIVERELOAD_PATH}/lib/rack/livereload/processing_skip_analyzer.rb      2019-11-29 22:43:58.691183891 +0900
@@ -37,7 +37,7 @@
       end

       def html?
-        @headers['Content-Type'] =~ %r{text/html}
+        @headers['Content-Type'] =~ %r{text/html|application/xhtml\+xml}
       end

       def get?
__EOF__
