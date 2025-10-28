
file(GLOB SUNIT_FILES ${CMAKE_CURRENT_SOURCE_DIR}/* )            
file(COPY ${SUNIT_FILES} DESTINATION .)            
add_custom_command( 
        OUTPUT SUnit.star  
        COMMAND pwd
        COMMAND            
        ${CMAKE_COMMAND} -E env LD_LIBRARY_PATH=${CMAKE_LIBRARY_OUTPUT_DIRECTORY}
        $<TARGET_FILE_DIR:gst-tool>/$<TARGET_FILE_NAME:gst-tool> gst-package 
        --kernel-directory ${CMAKE_BINARY_DIR}/kernel
        --test package.xml
         )
add_custom_target(sunit DEPENDS SUnit.star)
