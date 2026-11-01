import java.lang.annotation.*;

@Deprecated
open module com.example.app {
    requires java.base;
    requires transitive java.sql;
    requires static java.compiler;
    requires static transitive java.xml;
    requires module.with.dots;
    exports com.example.api;
    exports com.example.internal to com.example.a, com.example.b;
    opens com.example.model;
    opens com.example.impl to com.example.c;
    uses com.example.Service;
    provides com.example.Service with com.example.Impl1, com.example.Impl2;
    provides com.example.Other with com.example.OtherImpl;
}
